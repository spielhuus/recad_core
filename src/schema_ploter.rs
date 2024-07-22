use lazy_static::lazy_static;
use ndarray::{arr2, Array2, Axis};

use crate::{
    draw::At,
    gr::{Arc, Circle, Color, Effects, Font, GraphicItem, Polyline, Pos, Pt, Pts, Rect, Rectangle},
    math::{bbox::Bbox, pin_position, ToNdarray, Transform},
    plot::{
        theme::{Style, Theme},
        FontAnchor, FontBaseline, Paint, PlotCommand, Plotter,
    },
    schema::SchemaItem,
    sexp::constants::el,
    symbols::Pin,
    Error, Plot, Schema,
};

lazy_static! {
    static ref NO_CONNECT_R: Array2<f32> = arr2(&[
        [-el::NO_CONNECT_SIZE, -el::NO_CONNECT_SIZE],
        [el::NO_CONNECT_SIZE, el::NO_CONNECT_SIZE]
    ]);
    static ref NO_CONNECT_L: Array2<f32> = arr2(&[
        [-el::NO_CONNECT_SIZE, el::NO_CONNECT_SIZE],
        [el::NO_CONNECT_SIZE, -el::NO_CONNECT_SIZE]
    ]);
}

macro_rules! outline {
    ($self:expr, $item:expr, $plotter:expr) => {
        if cfg!(debug_assertions) {
            let outline = $item.outline(&$self)?;
            $plotter.move_to(outline.start);
            $plotter.line_to(Pt {
                x: outline.end.x,
                y: outline.start.y,
            });
            $plotter.line_to(outline.end);
            $plotter.line_to(Pt {
                x: outline.start.x,
                y: outline.end.y,
            });
            $plotter.line_to(outline.start);
            $plotter.close();
            $plotter.stroke(Paint::outline());
        }
    };
}

impl Plot for Schema {
    ///Move the cursor position to the pt.
    fn move_to(&mut self, pt: At) {
        self.last_pos = pt;
    }

    ///Resolve the At position to a Pt
    ///TODO is this the right place?
    fn get_pt(&self, at: &At) -> Pt {
        match at {
            At::Pt(pt) => *pt,
            At::Pin(reference, pin) => {
                let unit = self
                    .pin_unit(reference, pin)
                    .ok_or(Error(
                        "builder".to_string(),
                        format!("pin unit not found ({}, {})", reference, pin),
                    ))
                    .unwrap();
                let symbol = self
                    .symbol(reference, unit)
                    .ok_or(Error(
                        "builder".to_string(),
                        format!("symbol unit not found ({}, {})", reference, unit),
                    ))
                    .unwrap();
                let lib = self
                    .library_symbol(&symbol.lib_id)
                    .ok_or(Error(
                        "builder".to_string(),
                        format!("symbol unit not found ({}, {})", reference, unit),
                    ))
                    .unwrap();
                let pin = lib
                    .pin(pin)
                    .ok_or(Error(
                        "builder".to_string(),
                        format!("pin not found ({})", pin),
                    ))
                    .unwrap();
                pin_position(symbol, pin)
            }
            At::Dot(_) => todo!(),
        }
    }

    fn plot(&self, plotter: &mut impl Plotter, command: PlotCommand) -> Result<(), Error> {
        let theme = Theme::from(command.theme);
        let paper_size: (f32, f32) = self.paper.clone().into();

        for item in &self.items {
            match item {
                SchemaItem::Symbol(symbol) => {
                    outline!(self, symbol, plotter);
                    for prop in &symbol.props {
                        if prop.visible() {
                            outline!(self, prop, plotter);
                            let mut anchor = prop.effects.anchor();
                            plotter.text(
                                &prop.value,
                                prop.pos.into(), //TODO
                                Effects {
                                    font: Font {
                                        face: Some(theme.face()), //TODO
                                        size: theme
                                            .font_size(prop.effects.font.size, Style::Property),
                                        thickness: prop.effects.font.thickness,
                                        bold: prop.effects.font.bold,
                                        italic: prop.effects.font.italic,
                                        line_spacing: prop.effects.font.line_spacing,
                                        color: Some(
                                            theme.color(prop.effects.font.color, Style::Property),
                                        ),
                                    },
                                    justify: prop.effects.justify.clone(),
                                    hide: prop.visible(),
                                },
                                //FontEffects {
                                //    angle: if symbol.pos.angle + prop.pos.angle >= 360.0 {
                                //        symbol.pos.angle + prop.pos.angle - 360.0
                                //    } else if symbol.pos.angle + prop.pos.angle == 180.0 {
                                //        if anchor == FontAnchor::End {
                                //            anchor = FontAnchor::Start;
                                //        }
                                //        0.0
                                //    } else if symbol.pos.angle + prop.pos.angle == 90.0 {
                                //        270.0
                                //    } else {
                                //        symbol.pos.angle + prop.pos.angle
                                //    },
                                //    anchor,
                                //    baseline: prop.effects.baseline(),
                                //    face: theme.face(), //TODO prop.effects.font.face.clone().unwrap(),
                                //    size: theme
                                //        .font_size(prop.effects.font.size, Style::Property)
                                //        .0,
                                //    color: theme.color(prop.effects.font.color, Style::Property),
                                //},
                            );
                        }
                    }

                    let library = self.library_symbol(&symbol.lib_id).unwrap();
                    let transform = Transform::new()
                        .translation(symbol.pos.into())
                        .rotation(symbol.pos.angle)
                        .mirror(&symbol.mirror);

                    for lib_symbol in &library.units {
                        if lib_symbol.unit() == 0 || lib_symbol.unit() == symbol.unit {
                            for g in &lib_symbol.graphics {
                                match g {
                                    GraphicItem::Arc(a) => {
                                        arc(plotter, &transform, a, &Style::Outline, &theme);
                                    }
                                    GraphicItem::Polyline(p) => {
                                        polyline(plotter, &transform, p, &Style::Outline, &theme);
                                    }
                                    GraphicItem::Rectangle(p) => {
                                        rectangle(plotter, &transform, p, &Style::Outline, &theme);
                                    }
                                    GraphicItem::Circle(c) => {
                                        circle(plotter, &transform, c, &Style::Outline, &theme);
                                    }
                                    GraphicItem::Curve(_) => todo!(),
                                    GraphicItem::Line(_) => todo!(),
                                    GraphicItem::Text(_) => todo!(),
                                }
                            }
                        }
                    }
                    for p in &library.pins(symbol.unit) {
                        pin(
                            plotter,
                            &transform,
                            p,
                            library.pin_numbers,
                            library.pin_names,
                            library.pin_names_offset,
                            library.power,
                            &Style::Outline,
                            &theme,
                        );
                    }
                }
                SchemaItem::Wire(wire) => {
                    outline!(self, wire, plotter);
                    let pts1 = wire.pts.0.first().expect("pts[0] should exist");
                    let pts2 = wire.pts.0.get(1).expect("pts[0] should exist");
                    plotter.move_to(*pts1);
                    plotter.line_to(*pts2);
                    plotter.stroke(Paint {
                        color: theme.color(wire.stroke.color, Style::Wire),
                        fill: None,
                        width: theme.width(wire.stroke.width, Style::Wire),
                    });
                }
                SchemaItem::NoConnect(nc) => {
                    outline!(self, nc, plotter);
                    let transform = Transform::new().translation(nc.pos.into());
                    let r = transform.transform(&NO_CONNECT_R);
                    let l = transform.transform(&NO_CONNECT_L);

                    plotter.move_to(Pt {
                        x: r[[0, 0]],
                        y: r[[0, 1]],
                    });
                    plotter.line_to(Pt {
                        x: r[[1, 0]],
                        y: r[[1, 1]],
                    });
                    plotter.stroke(Paint {
                        color: theme.color(None, Style::NoConnect),
                        fill: None,
                        width: theme.width(0.0, Style::NoConnect),
                    });

                    plotter.move_to(Pt {
                        x: l[[0, 0]],
                        y: l[[0, 1]],
                    });
                    plotter.line_to(Pt {
                        x: l[[1, 0]],
                        y: l[[1, 1]],
                    });
                    plotter.stroke(Paint {
                        color: theme.color(None, Style::NoConnect),
                        fill: None,
                        width: theme.width(0.0, Style::NoConnect),
                    });
                }
                SchemaItem::Junction(junction) => {
                    outline!(self, junction, plotter);
                    plotter.circle(
                        junction.pos.into(),
                        if junction.diameter == 0.0 {
                            el::JUNCTION_DIAMETER / 2.0
                        } else {
                            junction.diameter / 2.0
                        },
                        Paint {
                            color: theme.color(None, Style::Junction),
                            fill: Some(theme.color(None, Style::Junction)),
                            width: theme.width(0.0, Style::Junction),
                        },
                    );
                }
                SchemaItem::LocalLabel(label) => {
                    outline!(self, label, plotter);
                    let text_pos: Array2<f32> = if label.pos.angle == 0.0 {
                        arr2(&[[label.pos.x + 1.0, label.pos.y]])
                    } else if label.pos.angle == 90.0 {
                        arr2(&[[label.pos.x, label.pos.y - 1.0]])
                    } else if label.pos.angle == 180.0 {
                        arr2(&[[label.pos.x - 1.0, label.pos.y]])
                    } else {
                        arr2(&[[label.pos.x, label.pos.y + 1.0]])
                    };
                    let text_angle = if label.pos.angle >= 180.0 {
                        label.pos.angle - 180.0
                    } else {
                        label.pos.angle
                    };
                    plotter.text(
                        &label.text,
                        Pos {
                            x: text_pos[[0, 0]],
                            y: text_pos[[0, 1]],
                            angle: label.pos.angle,
                        },
                        Effects {
                            font: Font {
                                face: Some(theme.face()), //TODO
                                size: theme.font_size(label.effects.font.size, Style::Property),
                                thickness: label.effects.font.thickness,
                                bold: label.effects.font.bold,
                                italic: label.effects.font.italic,
                                line_spacing: label.effects.font.line_spacing,
                                color: Some(theme.color(label.effects.font.color, Style::Property)),
                            },
                            justify: label.effects.justify.clone(),
                            hide: label.effects.hide,
                        },
                        //FontEffects {
                        //    angle: text_angle,
                        //    anchor: label.effects.anchor(),
                        //    baseline: label.effects.baseline(),
                        //    face: theme.face(), //TODO label.effects.font.face.clone().unwrap(),
                        //    size: theme.font_size(label.effects.font.size, Style::Label).0,
                        //    color: theme.color(label.effects.font.color, Style::Property),
                        //},
                    );
                }
                SchemaItem::GlobalLabel(label) => {
                    outline!(self, label, plotter);
                    //let angle: f64 = utils::angle(item.item).unwrap();
                    //let pos: Array1<f64> = utils::at(.item).unwrap();
                    let text_pos: Array2<f32> = if label.pos.angle == 0.0 {
                        arr2(&[[label.pos.x + 1.0, label.pos.y]])
                    } else if label.pos.angle == 90.0 {
                        arr2(&[[label.pos.x, label.pos.y - 1.0]])
                    } else if label.pos.angle == 180.0 {
                        arr2(&[[label.pos.x - 1.0, label.pos.y]])
                    } else {
                        arr2(&[[label.pos.x, label.pos.y + 1.0]])
                    };
                    let text_angle = if label.pos.angle >= 180.0 {
                        label.pos.angle - 180.0
                    } else {
                        label.pos.angle
                    };
                    plotter.text(
                        &label.text,
                        Pos {
                            x: text_pos[[0, 0]],
                            y: text_pos[[0, 1]],
                            angle: label.pos.angle,
                        },
                        Effects {
                            font: Font {
                                face: Some(theme.face()), //TODO
                                size: theme.font_size(label.effects.font.size, Style::Property),
                                thickness: label.effects.font.thickness,
                                bold: label.effects.font.bold,
                                italic: label.effects.font.italic,
                                line_spacing: label.effects.font.line_spacing,
                                color: Some(theme.color(label.effects.font.color, Style::Property)),
                            },
                            justify: label.effects.justify.clone(),
                            hide: label.effects.hide,
                        },
                        //FontEffects {
                        //    angle: text_angle,
                        //    anchor: label.effects.anchor(),
                        //    baseline: label.effects.baseline(),
                        //    face: theme.face(), //TODO label.effects.font.face.clone().unwrap(),
                        //    size: theme.font_size(label.effects.font.size, Style::Label).0,
                        //    color: theme.color(label.effects.font.color, Style::Property),
                        //},
                    );

                    //if item.global {
                    //    let mut outline = LabelElement::make_label(size);
                    //    if angle != 0.0 {
                    //        let theta = angle.to_radians();
                    //        let rot = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);
                    //        outline = outline.dot(&rot);
                    //    }
                    //    outline = outline + pos.clone();
                    //    plot_items.push(PlotItem::Polyline(
                    //        10,
                    //        Polyline::new(
                    //            outline,
                    //            theme.get_stroke(
                    //                Stroke::new(),
                    //                &[Style::GlobalLabel, Style::Fill(FillType::Background)],
                    //            ),
                    //            Some(LineCap::Round),
                    //            None,
                    //        ),
                    //    ));
                    //}
                }
                SchemaItem::Text(text) => {
                    outline!(self, text, plotter);
                    plotter.text(
                        &text.text,
                        text.pos,
                        Effects {
                            font: Font {
                                face: Some(theme.face()), //TODO
                                size: theme.font_size(text.effects.font.size, Style::Property),
                                thickness: text.effects.font.thickness,
                                bold: text.effects.font.bold,
                                italic: text.effects.font.italic,
                                line_spacing: text.effects.font.line_spacing,
                                color: Some(theme.color(text.effects.font.color, Style::Property)),
                            },
                            justify: text.effects.justify.clone(),
                            hide: text.effects.hide,
                        },
                        //FontEffects {
                        //    angle: text.pos.angle,
                        //    anchor: text.effects.anchor(),
                        //    baseline: text.effects.baseline(),
                        //    face: theme.face(), //TODO label.effects.font.face.clone().unwrap(),
                        //    size: theme.font_size(text.effects.font.size, Style::Label).0,
                        //    color: theme.color(text.effects.font.color, Style::Property),
                        //},
                    );
                }
                _ => log::error!("plotting item not supported: {:?}", item),
            }
        }

        // apply the settings to the drawing
        plotter.scale(command.scale);
        if command.border {
            plotter.set_view_box(Rect {
                start: Pt { x: 0.0, y: 0.0 },
                end: Pt {
                    x: paper_size.0,
                    y: paper_size.1,
                },
            });
        } else {
            let outline = self.outline()?;
            plotter.set_view_box(Rect {
                start: Pt {
                    x: outline.start.x,
                    y: outline.start.y,
                },
                end: Pt {
                    x: outline.end.x,
                    y: outline.end.y,
                },
            });
        }

        if cfg!(debug_assertions) {
            let outline = self.outline()?;
            plotter.rect(
                Rect {
                    start: outline.start,
                    end: Pt {
                        x: outline.end.x - outline.start.x,
                        y: outline.end.y - outline.start.y,
                    },
                },
                Paint::red(),
            );
        }
        Ok(())
    }
}

fn polyline(
    //<P: Plotter>(
    plotter: &mut impl Plotter,
    transform: &Transform,
    poly: &Polyline,
    style: &Style,
    theme: &Theme,
) {
    let pts = transform.transform(&poly.pts.ndarray());
    for (i, p) in pts.axis_iter(Axis(0)).enumerate() {
        if i == 0 {
            plotter.move_to(Pt { x: p[0], y: p[1] });
        } else {
            plotter.line_to(Pt { x: p[0], y: p[1] });
        }
    }

    plotter.stroke(Paint {
        color: theme.color(poly.stroke.color, style.clone()),
        fill: match poly.fill {
            crate::gr::FillType::None => None,
            crate::gr::FillType::Background => Some(theme.fill(None, Style::Background)),
            crate::gr::FillType::Outline => Some(theme.fill(None, Style::Outline)),
            crate::gr::FillType::Color(color) => Some(color),
        },
        width: theme.width(poly.stroke.width, style.clone()),
    });
}

fn arc(
    //<P: Plotter>(
    plotter: &mut impl Plotter,
    transform: &Transform,
    arc: &Arc,
    style: &Style,
    theme: &Theme,
) {
    plotter.arc(
       transform.transform(&arc.start.ndarray()).ndarray(), 
       transform.transform(&arc.mid.ndarray()).ndarray(), 
       transform.transform(&arc.end.ndarray()).ndarray(),
        Paint {
            color: theme.color(None, style.clone()),
            fill: None,
            width: theme.width(0.0, style.clone()),
        });
}

fn rectangle<P: Plotter>(
    plotter: &mut P,
    transform: &Transform,
    rect: &Rectangle,
    style: &Style,
    theme: &Theme,
) {
    let rect = arr2(&[[rect.start.x, rect.start.y], [rect.end.x, rect.end.y]]);
    let t = transform.transform(&rect);

    let x = if t[[0, 0]] > t[[1, 0]] {
        t[[1, 0]]
    } else {
        t[[0, 0]]
    };
    let y = if t[[0, 1]] > t[[1, 1]] {
        t[[1, 1]]
    } else {
        t[[0, 1]]
    };
    let width = (t[[1, 0]] - t[[0, 0]]).abs();
    let height = (t[[1, 1]] - t[[0, 1]]).abs();
    plotter.rect(
        Rect {
            start: Pt { x, y },
            end: Pt {
                x: width,
                y: height,
            },
        },
        Paint {
            color: theme.color(None, style.clone()),
            fill: None,
            width: theme.width(0.0, style.clone()),
        },
    );
}

fn circle<P: Plotter>(
    plotter: &mut P,
    transform: &Transform,
    circle: &Circle,
    style: &Style,
    theme: &Theme,
) {
    let center = arr2(&[[circle.center.x, circle.center.y]]);
    let t = transform.transform(&center);
    plotter.circle(
        Pt {
            x: t[[0, 0]],
            y: t[[0, 1]],
        },
        circle.radius,
        Paint {
            color: theme.color(None, style.clone()),
            fill: None,
            width: theme.width(0.0, style.clone()),
        },
    );
}

#[allow(clippy::too_many_arguments)]
fn pin<P: Plotter>(
    plotter: &mut P,
    transform: &Transform,
    pin: &Pin,
    pin_numbers: bool,
    pin_names: bool,
    pin_names_offset: Option<f32>,
    power: bool,
    style: &Style,
    theme: &Theme,
) {
    let pin_line: Pts = Pts(vec![
        Pt { x: 0.0, y: 0.0 },
        Pt {
            x: if pin.pos.angle == 0.0 || pin.pos.angle == 180.0 {
                pin.length
            } else {
                -pin.length
            },
            y: 0.0,
        },
    ]);
    let transform_pin = Transform::new()
        .translation(Pt {
            x: pin.pos.x,
            y: pin.pos.y,
        })
        .rotation(pin.pos.angle);
    let pin_pts = transform_pin.transform(&pin_line.ndarray());
    let pts: Pts = transform.transform(&pin_pts).ndarray();
    //TODO draw differnt pin graphic types.
    //https://github.com/KiCad/kicad-source-mirror/blob/c36efec4b20a59e306735e5ecbccc4b59c01460e/eeschema/sch_pin.cpp#L245

    plotter.move_to(pts.0[0]);
    plotter.line_to(pts.0[1]);
    plotter.stroke(Paint {
        color: theme.color(None, style.clone()),
        fill: None,
        width: theme.width(0.0, style.clone()),
    });

    if pin_numbers && !power {
        let to = match pin.pos.angle {
            0.0 => Pt {
                x: pin.length / 2.0,
                y: -0.75,
            },
            90.0 => Pt {
                x: 0.0,
                y: pin.length / 2.0,
            },
            180.0 => Pt {
                x: -pin.length / 2.0,
                y: -0.75,
            },
            270.0 => Pt {
                x: 0.0,
                y: pin.length / 2.0,
            },
            _ => {
                panic!("pin angle: {}, not supported", pin.pos.angle);
            }
        };

        let translate = Transform::new()
            .translation(Pt { x: to.x, y: to.y })
            .mirror(&Some(String::from("x")));
        let line: Pts = translate.transform(&pts.ndarray()).ndarray();
        let pos = line.0[0];
        plotter.text(
            &pin.number.name,
            Pos {
                x: pos.x,
                y: pos.y,
                angle: 0.0,
            },
            Effects {
                font: Font {
                    face: Some(theme.face()), //TODO
                    size: theme.font_size((0.0, 0.0), Style::PinNumber),
                    color: Some(theme.color(None, Style::PinNumber)),
                    ..Default::default()
                },
                justify: vec![],
                hide: false,
            },
            //FontEffects {
            //    angle: 0.0,
            //    anchor: FontAnchor::Middle,
            //    baseline: FontBaseline::Middle,
            //    face: String::from("osifont"),
            //    size: 1.25,
            //    color: Color::black(),
            //},
        );
    }

    if pin_names && pin.name.name != "~" && !power {
        let Some(offset) = pin_names_offset else {
            return;
        };
        let (to, align) = match pin.pos.angle {
            0.0 => (Pt { x: offset, y: 0.0 }, FontAnchor::Start),
            90.0 => (Pt { x: 0.0, y: offset }, FontAnchor::Start),
            180.0 => (Pt { x: -offset, y: 0.0 }, FontAnchor::End),
            270.0 => (Pt { x: 0.0, y: offset }, FontAnchor::End),
            _ => {
                panic!("pin angle: {}, not supported", pin.pos.angle);
            }
        };
        let translate = Transform::new()
            .translation(Pt { x: to.x, y: to.y })
            .mirror(&Some(String::from("x")));
        let line: Pts = translate.transform(&pts.ndarray()).ndarray();
        let pos = line.0[0];
        plotter.text(
            &pin.name.name,
            Pos {
                x: pos.x,
                y: pos.y,
                angle: 0.0,
            },
            Effects {
                font: Font {
                    face: Some(theme.face()), //TODO
                    size: theme.font_size((0.0, 0.0), Style::PinName),
                    color: Some(theme.color(None, Style::PinName)),
                    ..Default::default()
                },
                justify: vec![],
                hide: false,
            },
            //FontEffects {
            //    angle: 0.0,
            //    anchor: align,
            //    baseline: FontBaseline::Middle,
            //    face: String::from("osifont"),
            //    size: 1.75,
            //    color: Color::red(),
            //},
        );
    }
}
