//!Drawers for building schemas.
use std::path::PathBuf;

use crate::{
    gr::{Pos, Pt, Pts},
    math,
    schema::{GlobalLabel, Junction, LocalLabel, SchemaItem, Symbol, Wire},
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
}

///Dot position
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DotPosition {
    Start,
    End,
}

///Direction enum
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
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

    ///Get the anchor attribute..
    pub fn anchor(&self) -> Option<String> {
        for i in &self.attributes {
            if let Attribute::Anchor(pin) = i {
                return Some(pin.to_string());
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
        let pt = self.get_pt(&self.last_pos);
        label.pos.x = pt.x;
        label.pos.y = pt.y;
        self.items.push(SchemaItem::LocalLabel(label));
        Ok(())
    }
}

impl Drawable<LocalLabel> for LocalLabel {
    fn rotate(mut self, angle: f32) -> LocalLabel {
        self.pos.angle = angle;
        self
    }

    fn len(self, len: f32) -> LocalLabel {
        self
    }

    fn up(self) -> LocalLabel {
        self
    }

    fn down(self) -> LocalLabel {
        self
    }

    fn left(self) -> LocalLabel {
        self
    }

    fn right(self) -> LocalLabel {
        self
    }

    fn anchor(self, pin: &str) -> LocalLabel {
        self
    }

    fn mirror(self, axis: &str) -> LocalLabel {
        todo!()
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
    fn rotate(mut self, angle: f32) -> GlobalLabel {
        self.pos.angle = angle;
        self
    }

    fn len(self, len: f32) -> GlobalLabel {
        todo!()
    }

    fn up(self) -> GlobalLabel {
        todo!()
    }

    fn down(self) -> GlobalLabel {
        todo!()
    }

    fn left(self) -> GlobalLabel {
        todo!()
    }

    fn right(self) -> GlobalLabel {
        todo!()
    }

    fn anchor(self, pin: &str) -> GlobalLabel {
        self
    }

    fn mirror(self, axis: &str) -> GlobalLabel {
        todo!()
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
    fn rotate(self, angle: f32) -> Wire {
        self
    }

    fn len(mut self, len: f32) -> Wire {
        self.attrs.push(Attribute::Length(len));
        self
    }

    fn up(mut self) -> Wire {
        self.attrs.push(Attribute::Direction(Direction::Up));
        self
    }

    fn down(mut self) -> Wire {
        self.attrs.push(Attribute::Direction(Direction::Down));
        self
    }

    fn left(mut self) -> Wire {
        self.attrs.push(Attribute::Direction(Direction::Left));
        self
    }

    fn right(mut self) -> Wire {
        self.attrs.push(Attribute::Direction(Direction::Right));
        self
    }

    fn anchor(self, pin: &str) -> Wire {
        self
    }

    fn mirror(self, axis: &str) -> Wire {
        todo!()
    }
}

impl Drawer<Wire> for Schema {
    fn draw(&mut self, mut wire: Wire) -> Result<(), Error> {
        let pt = self.get_pt(&self.last_pos);
        let to_pos = match wire.attrs.direction() {
            Direction::Left => Pt {
                x: pt.x - wire.attrs.length().unwrap_or(self.grid) * self.grid,
                y: pt.y,
            },
            Direction::Right => Pt {
                x: pt.x + wire.attrs.length().unwrap_or(self.grid) * self.grid,
                y: pt.y,
            },
            Direction::Up => Pt {
                x: pt.x,
                y: pt.y - wire.attrs.length().unwrap_or(self.grid) * self.grid,
            },
            Direction::Down => Pt {
                x: pt.x,
                y: pt.y + wire.attrs.length().unwrap_or(self.grid) * self.grid,
            },
        };

        wire.pts = Pts(vec![pt, to_pos]);
        //TODO uuid: crate::uuid!(),

        self.items.push(SchemaItem::Wire(wire));
        self.last_pos = At::Pt(to_pos);
        Ok(())
    }
}

//pub struct Symbol {
//    pub reference: String,
//    pub value: String,
//    pub lib_id: String,
//    pub unit: u8,
//    pub angle: f32,
//    pub mirror: Option<String>,
//    pub anchor: String,
//    pub attrs: To,
//}

impl Drawable<Symbol> for Symbol {
    fn rotate(mut self, angle: f32) -> Symbol {
        self.attrs.push(Attribute::Rotate(angle));
        self
    }

    fn len(self, len: f32) -> Symbol {
        todo!()
    }

    fn up(self) -> Symbol {
        todo!()
    }

    fn down(self) -> Symbol {
        todo!()
    }

    fn left(self) -> Symbol {
        todo!()
    }

    fn right(self) -> Symbol {
        todo!()
    }

    fn anchor(mut self, pin: &str) -> Symbol {
        self.attrs.push(Attribute::Anchor(pin.to_string()));
        self
    }

    fn mirror(mut self, axis: &str) -> Symbol {
        self.attrs.push(Attribute::Mirror(axis.to_string()));
        self
    }
    //fn rotate(mut self, angle: f32) -> Self {
    //    self.angle = angle;
    //    self
    //}
    //pub fn mirror(mut self, mirror: &str) -> Self {
    //    self.mirror = Some(mirror.to_string());
    //    self
    //}
    //pub fn anchor(mut self, pin: &str) -> Self {
    //    self.anchor = pin.to_string();
    //    self
    //}
    //pub fn unit(mut self, unit: u8) -> Self {
    //    self.unit = unit;
    //    self
    //}
}

impl Symbol {
    pub fn tox(mut self, reference: &str, pin: &str) -> Self {
        self.attrs.push(Attribute::Tox(At::Pin(
            reference.to_string(),
            pin.to_string(),
        )));
        self
    }
    //pub fn len(mut self, len: f32) -> Self {
    //    self.len = len;
    //    self
    //}
    //pub fn up(mut self) -> Self {
    //    self.attrs.push(Attribute::Direction(Direction::Up));
    //    self
    //}
    //pub fn down(mut self) -> Self {
    //    self.attrs.push(Attribute::Direction(Direction::Down));
    //    self
    //}
    //pub fn left(mut self) -> Self {
    //    self.attrs.push(Attribute::Direction(Direction::Left));
    //    self
    //}
    //pub fn right(mut self) -> Self {
    //    self.attrs.push(Attribute::Direction(Direction::Right));
    //    self
    //}
}

impl Drawer<Symbol> for Schema {
    fn draw(&mut self, symbol: Symbol) -> Result<(), Error> {
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

        //create the new symbol
        let mut new_symbol = lib.symbol(symbol.unit);
        new_symbol.pos.angle = symbol.attrs.angle().unwrap_or(0.0);
        new_symbol.mirror = symbol.mirror.clone();

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
                format!("anchor pin not found: {}:{}", symbol.property(el::PROPERTY_REFERENCE), anchor),
            ))?,
        );

        //for attr in &symbol.attrs.attributes {
        //    println!("{:?}", attr);
        //    match attr {
        //        Attribute::Anchor(_) => todo!(),
        //        Attribute::Direction(_) => todo!(),
        //        Attribute::Id(_) => todo!(),
        //        Attribute::Mirror(_) => todo!(),
        //        Attribute::Length(_) => todo!(),
        //        Attribute::Rotate(_) => todo!(),
        //        Attribute::Tox(at) => {
        //            //match at {
        //            //    At::Pt(_) => todo!(),
        //            //    At::Pin(reference, pin) => {
        //            //        let to_symbol = self.symbol(reference, unit)
        //            //        let target_pos = crate::math::pin_position(&new_symbol, self.pin(&reference, &pin).unwrap());
        //            //        println!("  To({:?})", target_pos);
        //            //    },
        //            //    At::Dot(_) => todo!(),
        //            //}
        //        }
        //        Attribute::Toy(_) => todo!(),
        //        Attribute::Property(_) => todo!(),
        //        Attribute::Dot(_) => todo!(),
        //    }
        //}
        //
        //calculate position
        let pt = self.get_pt(&self.last_pos);
        let start_pt = Pt {
            x: pt.x - pin_pos.x,
            y: pt.y - pin_pos.y,
        };

        new_symbol.pos.x = start_pt.x;
        new_symbol.pos.y = start_pt.y;

        //set the properties
        new_symbol.set_property(
            el::PROPERTY_REFERENCE,
            symbol.property(el::PROPERTY_REFERENCE).as_str(),
        );
        new_symbol.set_property(
            el::PROPERTY_VALUE,
            symbol.property(el::PROPERTY_REFERENCE).as_str(),
        );

        //create the pins
        for pin in &lib.pins(symbol.unit) {
            new_symbol
                .pins
                .push((pin.number.name.clone(), crate::uuid!()));
        }

        math::place_properties(&lib, &mut new_symbol);

        //TODO the next pin should be pin 2
        self.last_pos = At::Pt(crate::math::pin_position(
            &new_symbol,
            lib.pin("2").unwrap(),
        ));
        self.items.push(SchemaItem::Symbol(new_symbol));
        Ok(())
    }
}
