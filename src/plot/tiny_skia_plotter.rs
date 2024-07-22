use std::{fs::File, io::Write};

use tiny_skia::{BlendMode, FillRule, Pixmap};

use crate::{
    gr::{Color, Effects, Pos, Pt, Pts, Rect},
    math::fonts::rasterize,
};

use super::{Paint, Plotter, PlotterImpl};

pub const SCALE: f32 = 25.4 * 0.72;

fn to_color(color: &Color) -> tiny_skia::Color {
    let (r, g, b, a) = match color {
        Color::None => todo!(),
        Color::Rgb(r, g, b) => (*r, *g, *b, 255),
        Color::Rgba(r, g, b, a) => (*r, *g, *b, *a),
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

    fn save(self, path: &std::path::Path) -> std::io::Result<()> {
        let mut buffer: Vec<u8> = Vec::new();
        self.write(&mut buffer)?;
        let mut file = File::create(path)?;
        file.write_all(buffer.as_slice())
    }

    fn write<W: std::io::Write>(mut self, writer: &mut W) -> std::io::Result<()> {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(0, 127, 0, 200);
        paint.anti_alias = true;

        let mut dt = if let Some(viewbox) = &self.viewbox {
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
                    start,
                    mid,
                    end,
                    stroke,
                } => {



                    //let center = center();
                    //
                    let mut path =
                        tiny_skia::PathBuilder::new();
                    
                    let ctrl = calculate_control_point(*start, *end, *mid);


                    path.move_to(start.x, start.y);
                    path.cubic_to(ctrl.x, ctrl.y, mid.x, mid.y, end.x, end.y);

                    //
                    // let segments = 100; // Number of segments to approximate the arc
                    //
                    //// Add the starting point
                    //path.move_to(
                    //    cx + radius * start_angle.cos(),
                    //    cy + radius * start_angle.sin(),
                    //);
                    //
                    //// Compute the points of the arc
                    //for i in 0..=segments {
                    //    let angle = start_angle + sweep_angle * (i as f32 / segments as f32);
                    //    let x = cx + radius * angle.cos();
                    //    let y = cy + radius * angle.sin();
                    //    path_builder.line_to(x, y);
                    //}


                    //path.move_to(start.x, start.y);
                    //path.quad_to(mid.x, mid.y, end.x, end.y);
                    let path = path.finish().unwrap();
                    if let Some(fill) = stroke.fill {
                        do_fill!(dt, &path, fill);
                    }
                    do_stroke!(dt, &path, stroke);



                    let path =
                        tiny_skia::PathBuilder::from_circle(start.x, start.y, 0.1).unwrap();
                    do_fill!(dt, &path, Color::blue());
                    do_stroke!(dt, &path, Paint::blue());
                    let path =
                        tiny_skia::PathBuilder::from_circle(mid.x, mid.y, 0.1).unwrap();
                    do_fill!(dt, &path, Color::green());
                    do_stroke!(dt, &path, Paint::green());
                    let path =
                        tiny_skia::PathBuilder::from_circle(end.x, end.y, 0.1).unwrap();
                    do_fill!(dt, &path, Color::red());
                    do_stroke!(dt, &path, Paint::red());


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
                super::PlotterNodes::Text { text, pos, effects } => {
                    let glyphs = rasterize(text, pos, effects).unwrap();
                    for g in glyphs { 
                        //TODO align
                        let mut pixmap = Pixmap::new(g.width, g.height).unwrap();
                        pixmap.data_mut().copy_from_slice(&g.data);
                        dt.draw_pixmap(
                            g.x as i32,
                            g.y as i32,
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

        let res = dt.encode_png()?;
        writer.write_all(res.as_slice())?;
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
}

fn calculate_control_point(start: Pt, end: Pt, mid: Pt) -> Pt {
    let start_to_mid = mid - start;
    let end_to_mid = mid - end;
    end + start_to_mid * 2.0 - end_to_mid
}
