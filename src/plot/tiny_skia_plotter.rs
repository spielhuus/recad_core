use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle};
use tiny_skia::{BlendMode, FillRule, Pixmap};

use crate::{
    gr::{Color, Pt, Pts, Rect},
    math::fonts::OSIFONT,
};

use super::{text::load_font, FontEffects, Paint, Plotter, PlotterImpl};

const SCALE: f32 = 25.4 * 0.72;

fn to_color(color: &Color) -> tiny_skia::Color {
    let (r, g, b, a) = match color {
        Color::None => todo!(),
        Color::Rgb(r, g, b) => (*r, *g, *b, 255),
        Color::Rgba(r, g, b, a) => (*r, *g, *b, 255),
    };
    tiny_skia::Color::from_rgba8(r, g, b, a)
}

macro_rules! do_stroke {
    ($dt:expr, $path:expr, $stroke:expr) => {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(to_color(&$stroke.color));
        $dt.stroke_path(
            &$path,
            &paint,
            &tiny_skia::Stroke {
                width: $stroke.width * SCALE,
                ..Default::default()
            },
            tiny_skia::Transform::identity(),
            None,
        );
    };
}

macro_rules! do_fill {
    ($dt:expr, $path:expr, $color:expr) => {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(to_color(&$color));
        $dt.fill_path(
            &$path,
            &paint,
            FillRule::Winding,
            tiny_skia::Transform::identity(),
            None,
        );
    };
}

///Plot a schema/pcb to a svg file.
pub struct TinySkiaPlotter {
    viewbox: Option<Rect>,
    scale: f32,
    cache: PlotterImpl,
}

#[allow(clippy::new_without_default)]
impl TinySkiaPlotter {
    pub fn new() -> Self {
        TinySkiaPlotter {
            viewbox: None,
            scale: 1.0,
            cache: PlotterImpl::new(),
        }
    }
}

impl Plotter for TinySkiaPlotter {
    fn open(&self) {
        panic!("open not supported for TinySkiaPlotter")
    }

    fn save(mut self, path: &std::path::Path) -> std::io::Result<()> {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(0, 127, 0, 200);
        paint.anti_alias = true;

        let mut dt = if let Some(viewbox) = self.viewbox {
            tiny_skia::Pixmap::new(
                (viewbox.end.x * SCALE) as u32,
                (viewbox.end.y * SCALE) as u32,
            )
            .unwrap()
        } else {
            tiny_skia::Pixmap::new((297.0 * SCALE) as u32, (210.0 * SCALE) as u32).unwrap()
        };
        let mut pb = tiny_skia::PathBuilder::new();

        self.cache.scale(SCALE);

        for item in self.cache.iter() {
            match item {
                super::PlotterNodes::MoveTo(pt) => pb.move_to(pt.x, pt.y),
                super::PlotterNodes::LineTo(pt) => pb.line_to(pt.x, pt.y),
                super::PlotterNodes::Close => pb.close(),
                super::PlotterNodes::Stroke(stroke) => {
                    let path = pb.finish().unwrap();
                    if let Some(fill) = stroke.fill {
                        do_fill!(dt, &path, fill);
                    }
                    do_stroke!(dt, &path, stroke);
                    pb = tiny_skia::PathBuilder::new();
                }
                super::PlotterNodes::Rect { rect, stroke } => {
                    if let Some(rect) = tiny_skia::Rect::from_xywh(
                        rect.start.x,
                        rect.start.y,
                        if rect.end.x == 0.0 { 1.0 } else { rect.end.x },
                        if rect.end.y == 0.0 { 1.0 } else { rect.end.y },
                    ) {
                        let path = tiny_skia::PathBuilder::from_rect(rect);
                        if let Some(fill) = stroke.fill {
                            do_fill!(dt, &path, fill);
                        }
                        do_stroke!(dt, &path, stroke);
                    } else {
                        println!(
                            "Unknwon Rect: {} {} {} {}",
                            rect.start.x,
                            rect.start.y,
                            if rect.end.x == 0.0 { 1.0 } else { rect.end.x },
                            if rect.end.y == 0.0 { 1.0 } else { rect.end.y }
                        );
                    }
                }
                super::PlotterNodes::Arc {
                    center,
                    radius,
                    stroke,
                } => {
                    //pb.arc(center.x, center.y, *radius, 0.0, std::f32::consts::PI);
                    let path = pb.finish();
                    //do_stroke!(dt, &path, stroke);
                    pb = tiny_skia::PathBuilder::new();
                }
                super::PlotterNodes::Circle {
                    center,
                    radius,
                    stroke,
                } => {
                    let path =
                        tiny_skia::PathBuilder::from_circle(center.x, center.y, *radius).unwrap();
                    if let Some(fill) = stroke.fill {
                        do_fill!(dt, &path, fill);
                    }
                    do_stroke!(dt, &path, stroke);
                }
                super::PlotterNodes::Text { text, pt, effects } => {
                    let font = match load_font() {
                        Ok(font) => font,
                        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
                    };

                    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
                    layout.reset(&LayoutSettings {
                        x: pt.x,
                        y: pt.y,
                        horizontal_align: match effects.anchor {
                            super::FontAnchor::Start => HorizontalAlign::Left,
                            super::FontAnchor::End => HorizontalAlign::Right,
                            super::FontAnchor::Middle => HorizontalAlign::Center,
                        },
                        //vertical_align: todo!(),
                        ..Default::default()
                    });
                    layout.append(&[font.clone()], &TextStyle::new(text, effects.size * 1.3333 * SCALE, 0));

                    let (r, g, b) = match effects.color {
                        Color::None => (255, 255, 255),
                        Color::Rgb(r, g, b) => (r, g, b),
                        Color::Rgba(r, g, b, _) => (r, g, b),
                    };

                    for glyph in layout.glyphs() {
                        let mut buffer = vec![];
                        let (metrics, coverage) = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px);
                        for lightness in coverage.into_iter() {
                            buffer.push(r);
                            buffer.push(g);
                            buffer.push(b);
                            buffer.push(lightness);
                        }
                        if metrics.width == 0 {
                            break;
                        }
                        let mut pixmap = Pixmap::new(metrics.width as u32, metrics.height as u32).unwrap();
                        pixmap.data_mut().copy_from_slice(&buffer);
                        dt.draw_pixmap(
                            glyph.x as i32,
                            glyph.y as i32,
                            pixmap.as_ref(),
                            &tiny_skia::PixmapPaint {
                                blend_mode: BlendMode::SourceOver,
                                opacity: 1.0,
                                ..Default::default()
                            },
                            tiny_skia::Transform::identity(),
                            None,
                        );
                    }
                }
            }
        }

        dt.save_png(path).unwrap();
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

    fn arc(&mut self, center: Pt, radius: f32, stroke: Paint) {
        self.cache.arc(center, radius, stroke);
    }

    fn circle(&mut self, center: Pt, radius: f32, stroke: Paint) {
        self.cache.circle(center, radius, stroke);
    }

    fn polyline(&mut self, pts: Pts, stroke: Paint) {
        self.cache.polyline(pts, stroke);
    }

    fn text(&mut self, text: &str, pt: Pt, effects: FontEffects) {
        self.cache.text(text, pt, effects);
    }
}
