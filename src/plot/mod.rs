//!Plot the recad drawings.
use std::{fmt, io::Write};

use crate::
    gr::{Color, Pt, Pts, Rect}
;

mod svg;
pub mod theme;

pub use svg::SvgPlotter;
use theme::Themes;

///The paint for the plotter.
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

///The fot effects for the drawings.
pub struct FontEffects {
    pub angle: f32,
    pub anchor: String,
    pub baseline: String,
    pub face: String,
    pub size: f32,
    pub color: Color,
}

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
    fn arc(&mut self, center: Pt, radius: f32, stroke: Paint);
    fn circle(&mut self, center: Pt, radius: f32, stroke: Paint);
    fn text(&mut self, text: &str, pt: Pt, effects: FontEffects);

    ///Draw a polyline with the given Pts.
    fn polyline(&mut self, pts: Pts, stroke: Paint);

    ///Write the result to a Writer.
    fn write<W: Write>(self, writer: &mut W) -> std::io::Result<()>;
}

