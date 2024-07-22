//!Plot the recad drawings.
use std::{fmt, io::Write, path::Path};

use crate::{
    gr::{Color, Effects, Pos, Pt, Pts, Rect},
    math::{ToNdarray, Transform},
};

mod femtovg;
mod raqote_plotter;
mod tiny_skia_plotter;
mod svg;
mod text;
pub mod theme;


pub use femtovg::FemtoVgPlotter;
pub use raqote_plotter::RaqotePlotter;
pub use tiny_skia_plotter::TinySkiaPlotter;
pub use svg::SvgPlotter;
use theme::Themes;

///The paint for the plotter.
///TODO use gr::Stroke
#[derive(Clone)]
pub struct Paint {
    pub(crate) color: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) width: f32,
}

impl Paint {
    pub fn black() -> Self {
        Self {
            color: Color::black(),
            fill: None,
            width: 0.25,
        }
    }
    pub fn red() -> Self {
        Self {
            color: Color::red(),
            fill: None,
            width: 0.25,
        }
    }
    pub fn green() -> Self {
        Self {
            color: Color::green(),
            fill: None,
            width: 0.25,
        }
    }
    pub fn blue() -> Self {
        Self {
            color: Color::blue(),
            fill: None,
            width: 0.25,
        }
    }
    pub fn grey() -> Self {
        Self {
            color: Color::grey(),
            fill: None,
            width: 0.25,
        }
    }
    pub fn outline() -> Self {
        Self {
            color: Color::red(),
            fill: None,
            width: 0.08,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontAnchor {
    Start,
    End,
    Middle,
}

impl fmt::Display for FontAnchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontAnchor::Start => write!(f, "start"),
            FontAnchor::End => write!(f, "end"),
            FontAnchor::Middle => write!(f, "middle"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontBaseline {
    Auto,
    Hanging,
    Middle,
}

impl fmt::Display for FontBaseline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontBaseline::Auto => write!(f, "auto"),
            FontBaseline::Hanging => write!(f, "hanging"),
            FontBaseline::Middle => write!(f, "middle"),
        }
    }
}

/////The font effects for the drawings.
//#[derive(Clone)]
//pub struct FontEffects {
//    pub angle: f32,
//    pub anchor: FontAnchor,
//    pub baseline: FontBaseline,
//    pub face: String,
//    pub size: f32,
//    pub color: Color,
//}

#[derive(Debug)]
//Line CAP, endings.
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl fmt::Display for LineCap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineCap::Butt => write!(f, "butt"),
            LineCap::Round => write!(f, "round"),
            LineCap::Square => write!(f, "square"),
        }
    }
}

/// Configure the Plotter
pub struct PlotCommand {
    pub border: bool,
    pub theme: Themes,
    pub scale: f32,
    pub pages: Vec<u8>,
    pub split: bool,
}

impl Default for PlotCommand {
    fn default() -> Self {
        PlotCommand {
            border: false,
            theme: Themes::Kicad2020,
            scale: 1.0,
            pages: Vec::new(),
            split: false,
        }
    }
}

impl PlotCommand {
    pub fn new() -> Self {
        PlotCommand {
            border: false,
            theme: Themes::Kicad2020,
            scale: 1.0,
            pages: Vec::new(),
            split: false,
        }
    }

    /// This function, when invoked, enables the plotter to append a border and a title to visuals. Upon deactivation,
    /// it trims the visual to encompass solely its substance.
    pub fn border(mut self, value: Option<bool>) -> Self {
        if let Some(value) = value {
            self.border = value;
        }
        self
    }

    /// This function sets the color theme for the plotter to interpret.
    pub fn theme(mut self, theme: Option<Themes>) -> Self {
        if let Some(theme) = theme {
            self.theme = theme;
        }
        self
    }

    /// This function allows you to adjust the dimensions of your visual content.
    /// Expandability only occurs in the absence of borders.
    pub fn scale(mut self, scale: Option<f32>) -> Self {
        if let Some(scale) = scale {
            self.scale = scale;
        }
        self
    }

    /// Selects the pages to plot; if the list is empty, all available pages will be plotted.
    pub fn pages<T>(mut self, pages: Option<T>) -> Self
    where
        T: Into<Vec<u8>>,
    {
        if let Some(pages) = pages {
            self.pages = pages.into();
        }
        self
    }

    pub fn split(mut self, value: Option<bool>) -> Self {
        if let Some(value) = value {
            self.split = value;
        }
        self
    }
}

pub trait Plotter {
    fn open(&self);

    ///set the view box.
    fn set_view_box(&mut self, rect: Rect);
    /// scale the image
    fn scale(&mut self, scale: f32);
    ///Move the path cursor to position.
    fn move_to(&mut self, pt: Pt);
    ///Draw a line to position.
    fn line_to(&mut self, pt: Pt);
    ///Close the path.
    fn close(&mut self);
    ///Sroke the path.
    fn stroke(&mut self, stroke: Paint);

    ///Draw a rectancle with stroke.
    fn rect(&mut self, r: Rect, stroke: Paint);
    ///Draw an arc with stroke.
    fn arc(&mut self, start: Pt, mid: Pt, end: Pt, stroke: Paint);
    fn circle(&mut self, center: Pt, radius: f32, stroke: Paint);
    fn text(&mut self, text: &str, pos: Pos, effects: Effects);

    ///Draw a polyline with the given Pts.
    fn polyline(&mut self, pts: Pts, stroke: Paint);


    /// Write the image to a buffer.
    fn write<W: Write>(self, writer: &mut W) -> std::io::Result<()>;

    /// Save the image to a path.
    fn save(self, path: &std::path::Path) -> std::io::Result<()>; //{
    //    let mut buffer: Vec<u8> = Vec::new();
    //    self.write(&mut buffer)?;
    //    let mut file = File::create(path)?;
    //    file.write_all(buffer.as_slice())
    //}
}

pub enum PlotterNodes {
    MoveTo(Pt),
    LineTo(Pt),
    Close,
    Stroke(Paint),
    Rect {
        rect: Rect,
        stroke: Paint,
    },
    Arc {
        start: Pt,
        mid: Pt,
        end: Pt,
        stroke: Paint,
    },
    Circle {
        center: Pt,
        radius: f32,
        stroke: Paint,
    },
    Text {
        text: String,
        pos: Pos,
        effects: Effects,
    },
}

/// Stores events from plotter sources for efficient access.
///
/// The event stream with callback functions employed in recad may not be optimal for working with plotter
/// libraries, as they often leverage implementations based on the builder pattern. This necessitates the
/// ability to manipulate variables, which can prove challenging when the variable is a struct member due
/// to its mutable nature. The proposed implementation will cache all events and provide them as an
/// iterator for easier access and manipulation.
///
/// TODO code example
pub struct PlotterImpl {
    items: Vec<PlotterNodes>,
}

impl PlotterImpl {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PlotterImpl { items: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<PlotterNodes> {
        self.items.iter()
    }

    pub fn scale(&mut self, scale: f32) {
        let transform = Transform::new().scale(scale);
        self.items = self.items.iter().map(|item| match item {
            PlotterNodes::MoveTo(pt) => {
                PlotterNodes::MoveTo(transform.transform(&pt.ndarray()).ndarray())
            }
            PlotterNodes::LineTo(pt) => {
                PlotterNodes::LineTo(transform.transform(&pt.ndarray()).ndarray())
            }
            PlotterNodes::Close => PlotterNodes::Close,
            PlotterNodes::Stroke(stroke) => PlotterNodes::Stroke(stroke.clone()),
            PlotterNodes::Rect { rect, stroke } => PlotterNodes::Rect {
                rect: Rect {
                    start: transform.transform(&rect.start.ndarray()).ndarray(),
                    end: transform.transform(&rect.end.ndarray()).ndarray(),
                },
                stroke: stroke.clone(),
            },
            PlotterNodes::Arc {
                start,
                mid,
                end,
                stroke,
            } => PlotterNodes::Arc {
                start: transform.transform(&start.ndarray()).ndarray(),
                mid: transform.transform(&mid.ndarray()).ndarray(),
                end: transform.transform(&end.ndarray()).ndarray(),
                stroke: stroke.clone(),
            },
            PlotterNodes::Circle {
                center,
                radius,
                stroke,
            } => PlotterNodes::Circle {
                center: transform.transform(&center.ndarray()).ndarray(),
                radius: radius * scale,
                stroke: stroke.clone(),
            },
            PlotterNodes::Text { text, pos, effects } => {
                let position: Pt = transform.transform(&pos.ndarray()).ndarray();
                PlotterNodes::Text {
                    text: text.clone(), 
                    pos: Pos { x: position.x, y: position.y, angle: pos.angle },
                    effects: effects.clone(),
                }
            },
        }).collect::<Vec<PlotterNodes>>();
    }
}

impl Plotter for PlotterImpl {
    fn open(&self) {}
    fn set_view_box(&mut self, _: Rect) {}
    fn scale(&mut self, _: f32) {}
    fn save(self, _: &Path) -> std::io::Result<()> { Ok(()) }
    fn write<W: Write>(self, _: &mut W) -> std::io::Result<()> { Ok(()) }

    fn move_to(&mut self, pt: Pt) {
        self.items.push(PlotterNodes::MoveTo(pt));
    }

    fn line_to(&mut self, pt: Pt) {
        self.items.push(PlotterNodes::LineTo(pt));
    }

    fn close(&mut self) {
        self.items.push(PlotterNodes::Close);
    }

    fn stroke(&mut self, stroke: Paint) {
        self.items.push(PlotterNodes::Stroke(stroke));
    }

    fn rect(&mut self, rect: Rect, stroke: Paint) {
        self.items.push(PlotterNodes::Rect { rect, stroke });
    }

    fn arc(&mut self, start: Pt, mid: Pt, end: Pt, stroke: Paint) {
        self.items.push(PlotterNodes::Arc {
            start,
            mid,
            end,
            stroke,
        });
    }

    fn circle(&mut self, center: Pt, radius: f32, stroke: Paint) {
        self.items.push(PlotterNodes::Circle {
            center,
            radius,
            stroke,
        });
    }

    fn text(&mut self, text: &str, pos: Pos, effects: Effects) {
        self.items.push(PlotterNodes::Text {
            text: text.to_string(),
            pos,
            effects,
        });
    }

    fn polyline(&mut self, pts: Pts, stroke: Paint) {
        let mut first: bool = true;
        for pos in pts.0 {
            if first {
                self.move_to(pos);
                first = false;
            } else {
                self.line_to(pos);
            }
        }
        self.stroke(stroke);
    }

}
