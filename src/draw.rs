//!Drawers for building schemas.
use std::path::PathBuf;

use crate::{
    gr::{Justify, Pos, Pt, Pts},
    math::{self, pin_position},
    schema::{GlobalLabel, Instance, Junction, LocalLabel, NoConnect, SchemaItem, Symbol, Wire},
    sexp::constants::el,
    Drawable, Drawer, Error, Plot, Schema,
};

///Attributes for the elements.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    Anchor(String),
    Direction(Direction),
    Id(String),
    Mirror(String),
    Length(f32),
    Rotate(f32),
    Tox(At),
    Toy(At),
    Property(String),
    Dot(Vec<DotPosition>),
    At(At),
    Unit(u8),
}

///Dot position
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DotPosition {
    Start,
    End,
}

///Direction enum
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

///Draw a Wire from the actual posistion to position.
#[derive(Debug, Clone, PartialEq)]
pub struct To {
    ///The Attributes.
    pub attributes: Vec<Attribute>,
}

impl To {
    ///Create a new empty To.
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    pub fn push(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }

    ///Get the to attribute..
    pub fn at(&self) -> Option<At> {
        for i in &self.attributes {
            if let Attribute::At(to) = i {
                return Some(to.clone());
            }
        }
        None
    }

    ///Get the anchor attribute..
    pub fn anchor(&self) -> Option<String> {
        for i in &self.attributes {
            if let Attribute::Anchor(pin) = i {
                return Some(pin.to_string());
            }
        }
        None
    }

    ///Get the anchor attribute..
    pub fn mirror(&self) -> Option<String> {
        for i in &self.attributes {
            if let Attribute::Mirror(m) = i {
                return Some(m.to_string());
            }
        }
        None
    }

    ///Get the angle attribute..
    pub fn angle(&self) -> Option<f32> {
        for i in &self.attributes {
            if let Attribute::Rotate(angle) = i {
                return Some(*angle);
            }
        }
        None
    }

    ///Get the length attribute..
    pub fn length(&self) -> Option<f32> {
        for i in &self.attributes {
            if let Attribute::Length(length) = i {
                return Some(*length);
            }
        }
        None
    }
    ///Get the direction.
    pub fn direction(&self) -> &Direction {
        for i in &self.attributes {
            if let Attribute::Direction(direction) = i {
                return direction;
            }
        }
        &Direction::Left
    }
    ///Get the tox position.
    pub fn tox(&self) -> Option<&At> {
        for i in &self.attributes {
            if let Attribute::Tox(at) = i {
                return Some(at);
            }
        }
        None
    }
    ///Get the toy position.
    pub fn toy(&self) -> Option<&At> {
        for i in &self.attributes {
            if let Attribute::Toy(at) = i {
                return Some(at);
            }
        }
        None
    }
    
    ///Get unit, only used for symbols.
    pub fn unit(&self) -> Option<u8> {
        for i in &self.attributes {
            if let Attribute::Unit(unit) = i {
                return Some(*unit);
            }
        }
        None
    }

    //Get the dot positions.
    pub fn dot(&self) -> Option<&Vec<DotPosition>> {
        for i in &self.attributes {
            if let Attribute::Dot(dot) = i {
                return Some(dot);
            }
        }
        None
    }
}

impl Default for To {
    fn default() -> Self {
        Self::new()
    }
}

///Represents different position identifiers
///
///Points can be different things.
///- the coordinates of a point.
///- the coordinates of a pin.
///- The coordinates of a previous element.
#[derive(Debug, Clone, PartialEq)]
pub enum At {
    ///A simple point with x and y in mm.
    Pt(Pt),
    ///The posiition of a ```Pin``` by refernce and pin number.
    Pin(String, String),
    ///TODO
    Dot(String),
}

impl Default for At {
    fn default() -> Self {
        At::Pt(Pt { x: 0.0, y: 0.0 })
    }
}

//TODO this should not be here
///implment the drawer functions for the schema.
impl Schema {}

impl Drawer<LocalLabel> for Schema {
    fn draw(&mut self, mut label: LocalLabel) -> Result<(), Error> {
        let pt = if let Some(at) = label.attrs.at() {
            self.get_pt(&at)
        } else {
            self.get_pt(&self.last_pos)
        };
        label.pos.x = pt.x;
        label.pos.y = pt.y;
        if let Some(angle) = label.attrs.angle() {
            label.pos.angle = angle;
        }
 
        // set the text adjustement
        if label.pos.angle == 0.0 || label.pos.angle == 90.0 {
            label.effects.justify = vec![Justify::Left, Justify::Bottom];
        } else if label.pos.angle == 180.0 || label.pos.angle == 270.0 {
            label.effects.justify = vec![Justify::Right, Justify::Bottom];
        }

        self.items.push(SchemaItem::LocalLabel(label));
        self.last_pos = At::Pt(pt);
        Ok(())
    }
}

impl Drawable<LocalLabel> for LocalLabel {
    fn attr(mut self, attr: Attribute) -> LocalLabel {
        self.attrs.push(attr);
        self
    }
}

impl Drawer<GlobalLabel> for Schema {
    fn draw(&mut self, mut label: GlobalLabel) -> Result<(), Error> {
        let pt = self.get_pt(&self.last_pos);
        label.pos.x = pt.x;
        label.pos.y = pt.y;
        self.items.push(SchemaItem::GlobalLabel(label));
        Ok(())
    }
}

impl Drawable<GlobalLabel> for GlobalLabel {
    fn attr(mut self, attr: Attribute) -> GlobalLabel {
        self.attrs.push(attr);
        self
    }
}

impl Drawer<Junction> for Schema {
    fn draw(&mut self, mut junction: Junction) -> Result<(), Error> {
        let pt = self.get_pt(&self.last_pos);
        junction.pos = Pos {
            x: pt.x,
            y: pt.y,
            angle: 0.0,
        };
        self.items.push(SchemaItem::Junction(junction));
        Ok(())
    }
}

impl Drawable<Wire> for Wire {
    fn attr(mut self, attr: Attribute) -> Wire {
        self.attrs.push(attr);
        self
    }
}

impl Drawer<Wire> for Schema {
    /// implement the draw function
    ///
    /// first the Tox and Toy attributes are searched
    /// otherwise a direction and length is used
    fn draw(&mut self, mut wire: Wire) -> Result<(), Error> {
        let pt = if let Some(to) = wire.attrs.at() {
            self.get_pt(&to)
        } else {
            self.get_pt(&self.last_pos)
        };

        let to_pos = if let Some(tox) = wire.attrs.tox() {
            let target_pos = self.get_pt(tox);
            Pt {
                x: target_pos.x,
                y: pt.y,
            }
        } else if let Some(toy) = wire.attrs.toy() {
            let target_pos = self.get_pt(toy);
            Pt {
                x: pt.x,
                y: target_pos.y,
            }
        } else {
            match wire.attrs.direction() {
                Direction::Left => Pt {
                    x: pt.x - wire.attrs.length().unwrap_or(self.grid),
                    y: pt.y,
                },
                Direction::Right => Pt {
                    x: pt.x + wire.attrs.length().unwrap_or(self.grid),
                    y: pt.y,
                },
                Direction::Up => Pt {
                    x: pt.x,
                    y: pt.y - wire.attrs.length().unwrap_or(self.grid),
                },
                Direction::Down => Pt {
                    x: pt.x,
                    y: pt.y + wire.attrs.length().unwrap_or(self.grid),
                },
            }
        };
        wire.pts = Pts(vec![pt, to_pos]);
        self.items.push(SchemaItem::Wire(wire));
        self.last_pos = At::Pt(to_pos);
        Ok(())
    }
}

impl Drawable<Symbol> for Symbol {
    fn attr(mut self, attr: Attribute) -> Symbol {
        self.attrs.push(attr);
        self
    }
}

impl Drawer<Symbol> for Schema {
    fn draw(&mut self, symbol: Symbol) -> Result<(), Error> {
        let mut new_last_pos = None;
        //load the library symbol
        let lib = if let Some(lib) = self.library_symbol(&symbol.lib_id) {
            lib.clone()
        } else {
            let lib = crate::SymbolLibrary {
                //TODO not finished
                pathlist: vec![PathBuf::from("/usr/share/kicad/symbols")],
            }
            .load(&symbol.lib_id)
            .unwrap();
            self.library_symbols.push(lib.clone());
            lib
        };

        let selected_unit = if let Some(unit) = symbol.attrs.unit() {
            unit
        } else {
            1
        };

        //create the new symbol
        let mut new_symbol = lib.symbol(selected_unit);
        new_symbol.pos.angle = symbol.attrs.angle().unwrap_or(0.0);
        new_symbol.mirror = symbol.attrs.mirror();
        new_symbol.unit = selected_unit;

        //create the transformer
        let anchor = if let Some(anchor) = symbol.attrs.anchor() {
            anchor
        } else {
            String::from("1")
        };

        let pin_pos = crate::math::pin_position(
            &new_symbol,
            lib.pin(&anchor).ok_or(Error(
                "drawer".to_string(),
                format!(
                    "anchor pin not found: {}:{}",
                    symbol.property(el::PROPERTY_REFERENCE),
                    anchor
                ),
            ))?,
        );

        if let Some(tox) = symbol.attrs.tox() {
            //TODO not finished
            let target_pos = self.get_pt(tox);
            let pin1 = pin_position(&new_symbol, lib.pin("1").unwrap());
            let pin2 = pin_position(&new_symbol, lib.pin("2").unwrap());
            let symbol_length = pin1.x - pin2.x;
            let pt = self.get_pt(&self.last_pos);
            let total_length = pt.x - target_pos.x;
            let wire_length = (total_length - symbol_length) / 2.0;

            self.draw(
                Wire::new()
                    .attr(Attribute::Direction(if wire_length < 0.0 { Direction::Right } else { Direction::Left }))
                    .attr(Attribute::Length(wire_length.abs())),
            )
            .unwrap();
            self.last_pos = At::Pt(Pt {
                x: target_pos.x,
                y: pt.y,
            });
            self.draw(
                Wire::new()
                    .attr(Attribute::Direction(if wire_length < 0.0 { Direction::Left } else { Direction::Right }))
                    .attr(Attribute::Length(wire_length.abs())),
            )
            .unwrap();

            new_last_pos = Some(At::Pt(Pt {
                x: target_pos.x,
                y: pt.y,
            }));
            self.last_pos = At::Pt(Pt {
                x: pt.x - wire_length,
                y: pt.y,
            })
        }

        //calculate position
        let pt = if let Some(to) = symbol.attrs.at() {
            self.get_pt(&to)
        } else {
            self.get_pt(&self.last_pos)
        };

        let start_pt = Pt {
            x: pt.x - pin_pos.x,
            y: pt.y - pin_pos.y,
        };

        new_symbol.pos.x = start_pt.x;
        new_symbol.pos.y = start_pt.y;

        //set the properties
        let reference = if new_symbol.property(el::PROPERTY_REFERENCE).starts_with("#PWR") {
            self.next_power()
        } else if new_symbol.property(el::PROPERTY_REFERENCE).starts_with("#FLG") {
            self.next_flag()
        } else {
            symbol.property(el::PROPERTY_REFERENCE)
        };
        new_symbol.set_property(
            el::PROPERTY_REFERENCE,
            reference.as_str(),
        );
        new_symbol.set_property(
            el::PROPERTY_VALUE,
            symbol.property(el::PROPERTY_VALUE).as_str(),
        );

        //create the pins
        for pin in &lib.pins(selected_unit) {
            new_symbol
                .pins
                .push((pin.number.name.clone(), crate::uuid!()));
        }

        math::place_properties(self, &mut new_symbol);

        //add the instances section
        new_symbol.instances = vec![
            Instance { 
                project: self.project.to_string(), 
                path: format!("/{}", self.uuid),
                reference: new_symbol.property(el::PROPERTY_REFERENCE), 
                unit: selected_unit,
            }
        ];

        if let Some(last_pos) = new_last_pos {
            self.last_pos = last_pos;
        } else {
            let pin_count = lib.pins(selected_unit).len();
            let out_pin = if pin_count == 1 || anchor == "2" {
                String::from("1")
            } else {
                String::from("2")
            };
            self.last_pos = At::Pt(crate::math::pin_position(
                &new_symbol,
                lib.pin(&out_pin).unwrap(),
            ));
        }
        self.items.push(SchemaItem::Symbol(new_symbol));
        Ok(())
    }
}

impl Drawer<NoConnect> for Schema {
    fn draw(&mut self, mut no_connect: NoConnect) -> Result<(), Error> {
        let pt = if let Some(at) = no_connect.attrs.at() {
            self.get_pt(&at)
        } else {
            self.get_pt(&self.last_pos)
        };
        no_connect.pos.x = pt.x;
        no_connect.pos.y = pt.y;
        self.items.push(SchemaItem::NoConnect(no_connect));
        self.last_pos = At::Pt(pt);
        Ok(())
    }
}

impl Drawable<NoConnect> for NoConnect {
    fn attr(mut self, attr: Attribute) -> NoConnect {
        self.attrs.push(attr);
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::{draw::{At, Attribute}, gr::Pt, schema::{LocalLabel, SchemaItem}, Drawable, Drawer, Schema};

    #[test]
    fn test_local_label() {
        let mut schema = Schema::new("test");
        let label = LocalLabel::new("INPUT")
            .attr(Attribute::Rotate(90.0))
            .attr(Attribute::At(At::Pt(Pt { x: 12.5, y: 12.5 })));

        schema.draw(label.clone()).unwrap();
        let Some(SchemaItem::LocalLabel(new_label)) = schema.items.last() else {
            panic!("label not found");
        };
        assert_eq!("INPUT", new_label.text);
        assert_eq!(12.5, new_label.pos.x);
        assert_eq!(12.5, new_label.pos.y);
        assert_eq!(90.0, new_label.pos.angle);



    }
}


