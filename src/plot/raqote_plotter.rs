
use raqote::{DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};

use crate::{gr::{Color, Effects, Pos, Pt, Pts, Rect}, math::fonts::OSIFONT};

use super::{Paint, Plotter, PlotterImpl};

const SCALE: f32 = 25.4;

fn to_color(color: &Color) -> (u8, u8, u8, u8) {
    match color {
        Color::None => todo!(),
        Color::Rgb(r, g, b) => (*r, *g, *b, 255),
        Color::Rgba(r, g, b, a) => (*r, *g, *b, *a),
    }
}

macro_rules! do_stroke {
    ($dt:expr, $path:expr, $stroke:expr) => {
        let (r, g, b, _a) = to_color(&$stroke.color);
        $dt.stroke(
            $path,
            &Source::Solid(SolidSource {
                r, g, b, a: 255,  //TODO what is with the a
            }),
            &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Round,
                width: $stroke.width * SCALE,
                ..StrokeStyle::default()
            },
            &DrawOptions::new()
        );
    };
}


///Plot a schema/pcb to a svg file.
pub struct RaqotePlotter {
    viewbox: Option<Rect>,
    scale: f32,
    cache: PlotterImpl,
}

#[allow(clippy::new_without_default)]
impl RaqotePlotter {
    pub fn new() -> Self {
        RaqotePlotter {
            viewbox: None,
            scale: 1.0,
            cache: PlotterImpl::new(),
        }
    }
}

impl Plotter for RaqotePlotter {
    fn open(&self) {
        panic!("open not supported for RaqotePlotter")
    }

    fn save(mut self, path: &std::path::Path) -> std::io::Result<()> {

        let mut dt = if let Some(viewbox) = self.viewbox {
            DrawTarget::new((viewbox.end.x * SCALE) as i32, (viewbox.end.y * SCALE) as i32)
        } else {
            DrawTarget::new((297.0 * SCALE) as i32,  (210.0 * SCALE) as i32)
        };
        let mut pb = PathBuilder::new();

        self.cache.scale(SCALE);

        for item in self.cache.iter() {
            match item {
                super::PlotterNodes::MoveTo(pt) => pb.move_to(pt.x, pt.y),
                super::PlotterNodes::LineTo(pt) => pb.line_to(pt.x, pt.y),
                super::PlotterNodes::Close => pb.close(),
                super::PlotterNodes::Stroke(stroke) => { 
                    let path = pb.finish(); 
                    do_stroke!(dt, &path, stroke);
                    pb = PathBuilder::new();
                },
                super::PlotterNodes::Rect { rect, stroke } => {
                    pb.rect(rect.start.x, rect.start.y, rect.end.x, rect.end.y);
                    let path = pb.finish(); 
                    do_stroke!(dt, &path, stroke);
                    pb = PathBuilder::new();
                },
                super::PlotterNodes::Arc { start, mid, end, stroke } => {
                    //TODO
                    //pb.arc(center.x, center.y, *radius, 0.0, std::f32::consts::PI);
                    //let path = pb.finish(); 
                    //do_stroke!(dt, &path, stroke);
                    //pb = PathBuilder::new();
                },
                super::PlotterNodes::Circle { center, radius, stroke } => {
                    pb.arc(center.x, center.y, *radius, 0.0, std::f32::consts::PI);
                    let path = pb.finish(); 
                    do_stroke!(dt, &path, stroke);
                    pb = PathBuilder::new();
                },
                super::PlotterNodes::Text { text, pos, effects } => {
                    let font = font_kit::loader::Loader::from_bytes(std::sync::Arc::new(OSIFONT.to_vec()), 0).unwrap();
                    dt.draw_text(
                        &font,
                        effects.font.size.0 * SCALE,
                        text,
                        raqote::Point::new(pos.x, pos.y),
                        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 180, 0)),
                        &DrawOptions::new(),
                    );
                    pb = PathBuilder::new();
                },
            }
        }
        dt.write_png(path)?;
        Ok(())
    }

    fn set_view_box(&mut self, rect: Rect) {
        self.viewbox = Some(rect)
    }

    fn scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn move_to(&mut self, pt: Pt) {
        self.cache.move_to(pt);
    }

    fn line_to(&mut self, pt: Pt) {
        self.cache.line_to(pt);
    }

    fn close(&mut self) {
        self.cache.close();
    }

    fn stroke(&mut self, stroke: Paint) {
        self.cache.stroke(stroke);
    }

    fn rect(&mut self, rect: Rect, stroke: Paint) {
        self.cache.rect(rect, stroke);
    }

    fn arc(&mut self, start: Pt, mid: Pt, end: Pt, stroke: Paint) {
        self.cache.arc(start, mid, end, stroke);
    }

    fn circle(&mut self, center: Pt, radius: f32, stroke: Paint) {
        self.cache.circle(center, radius, stroke);
    }

    fn polyline(&mut self, pts: Pts, stroke: Paint) {
        self.cache.polyline(pts, stroke);
    }

    fn text(&mut self, text: &str, pos: Pos, effects: Effects) {
        self.cache.text(text, pos, effects);
    }

    fn write<W: std::io::Write>(self, writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}
