use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, Paint, Path,
};
#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
#[cfg(not(target_arch = "wasm32"))]
use glutin_winit::DisplayBuilder;
use ndarray::arr2;
#[cfg(not(target_arch = "wasm32"))]
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::{
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::gr::{self, Pt, Rect};

use super::Plotter;

const SCALE: f32 = 25.4;

pub struct FemtoVgPlotter {
    viewbox: Option<Rect>,
    scale: f32,
    pathlist: Vec<(Path, Option<Paint>, Option<Paint>)>,
    textlist: Vec<(f32, f32, String, Option<Paint>)>,
    path: Path,
}

trait IntoFemto<T> {
    fn femto(&self) -> T;
}

impl IntoFemto<Color> for gr::Color {
    fn femto(&self) -> Color {
        match self {
            gr::Color::Rgb(r, g, b) => Color::rgb(*r, *g, *b),
            gr::Color::Rgba(r, g, b, a) => Color::rgba(*r, *g, *b, *a),
            _ => todo!(),
        }
    }
}

impl IntoFemto<Align> for Vec<gr::Justify> {
    fn femto(&self) -> Align {
        if self.contains(&gr::Justify::Right) {
            Align::Right
        } else if self.contains(&gr::Justify::Left) {
            Align::Left
        } else {
            Align::Center
        }
    }
}

impl IntoFemto<Baseline> for Vec<gr::Justify> {
    fn femto(&self) -> Baseline {
        if self.contains(&gr::Justify::Top) {
            Baseline::Top
        } else if self.contains(&gr::Justify::Bottom) {
            Baseline::Bottom
        } else {
            Baseline::Middle
        }
    }
}

impl Plotter for FemtoVgPlotter {
    fn open(&self) {
        self.do_open(297, 210, String::from("recad"), true);
    }

    fn set_view_box(&mut self, rect: Rect) {
        self.viewbox = Some(Rect {
            start: Pt {
                x: rect.start.x * SCALE,
                y: rect.start.y * SCALE,
            },
            end: Pt {
                x: rect.end.x * SCALE,
                y: rect.end.y * SCALE,
            },
        });
    }

    fn scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn move_to(&mut self, pt: crate::gr::Pt) {
        self.path.move_to(pt.x * SCALE, pt.y * SCALE);
    }

    fn line_to(&mut self, pt: crate::gr::Pt) {
        self.path.line_to(pt.x * SCALE, pt.y * SCALE);
    }

    fn close(&mut self) {
        self.path.close();
    }

    fn stroke(&mut self, stroke: super::Paint) {
        //let fstroke = stroke.color.map(|c| {
        let mut fstroke = Paint::color(stroke.color.femto());
        fstroke.set_line_width(stroke.width * SCALE);
        fstroke.set_anti_alias(true);
        //paint
        //});

        let fill = stroke
            .fill
            .map(|c| Paint::color(c.femto()).with_anti_alias(true));

        self.pathlist.push((self.path.clone(), fill, Some(fstroke)));
        self.path = Path::new();
    }

    fn rect(&mut self, r: Rect, stroke: super::Paint) {
        //let fstroke = stroke.color.map(|c| {
        let mut fstroke = Paint::color(stroke.color.femto());
        fstroke.set_line_width(stroke.width * SCALE);
        fstroke.set_anti_alias(true);
        //paint
        //});

        let fill = stroke
            .fill
            .map(|c| Paint::color(c.femto()).with_anti_alias(true));

        self.path.rect(
            r.start.x * SCALE,
            r.start.y * SCALE,
            r.end.x * SCALE,
            r.end.y * SCALE,
        );
        self.pathlist.push((self.path.clone(), fill, Some(fstroke)));
        self.path = Path::new();
    }

    fn arc(&mut self, center: crate::gr::Pt, radius: f32, stroke: super::Paint) {}

    fn circle(&mut self, center: crate::gr::Pt, radius: f32, stroke: super::Paint) {}

    fn text(&mut self, text: &str, pt: crate::gr::Pt, effects: super::FontEffects) {
        //let paint = effects.color.map(|c| {
        let mut paint = Paint::color(effects.color.femto());
        paint.set_font_size(effects.size * SCALE);
        paint.set_anti_alias(true);
        paint.set_text_align(Align::Center);
        paint.set_text_baseline(Baseline::Top);
        //paint
        //});

        self.textlist
            .push((pt.x * SCALE, pt.y * SCALE, text.to_string(), Some(paint)));
    }

    fn polyline(&mut self, pts: crate::gr::Pts, stroke: super::Paint) {
        let mut first: bool = true;
        for pt in pts.0 {
            if first {
                self.path.move_to(pt.x * SCALE, pt.y * SCALE);
                first = false;
            } else {
                self.path.line_to(pt.x * SCALE, pt.y * SCALE);
            }
        }
        self.stroke(stroke);
    }

    fn save(self, _path: &std::path::Path) -> std::io::Result<()> {
        todo!()
    }
}

impl FemtoVgPlotter {
    pub fn new() -> Self {
        Self {
            viewbox: None,
            scale: 1.0,
            pathlist: Vec::new(),
            textlist: Vec::new(),
            path: Path::new(),
        }
    }

    pub fn do_open(
        &self,
        #[cfg(not(target_arch = "wasm32"))] width: u32,
        #[cfg(not(target_arch = "wasm32"))] height: u32,
        #[cfg(not(target_arch = "wasm32"))] title: String,
        #[cfg(not(target_arch = "wasm32"))] resizeable: bool,
    ) {
        let event_loop = EventLoop::new();

        #[cfg(not(target_arch = "wasm32"))]
        let (canvas, window, context, surface) = {
            let window_builder = WindowBuilder::new()
                .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
                .with_resizable(resizeable)
                .with_title(title);

            let template = ConfigTemplateBuilder::new().with_alpha_size(8);
            let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

            let (window, gl_config) = display_builder
                .build(&event_loop, template, |configs| {
                    // Find the config with the maximum number of samples, so our triangle will
                    // be smooth.
                    configs
                        .reduce(|accum, config| {
                            let transparency_check =
                                config.supports_transparency().unwrap_or(false)
                                    & !accum.supports_transparency().unwrap_or(false);

                            if transparency_check || config.num_samples() < accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();

            let window = window.unwrap();

            let raw_window_handle = Some(window.raw_window_handle());

            let gl_display = gl_config.display();

            let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
            let fallback_context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::Gles(None))
                .build(raw_window_handle);
            let mut not_current_gl_context = Some(unsafe {
                gl_display
                    .create_context(&gl_config, &context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(&gl_config, &fallback_context_attributes)
                            .expect("failed to create context")
                    })
            });

            let (width, height): (u32, u32) = window.inner_size().into();
            let raw_window_handle = window.raw_window_handle();
            let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
                raw_window_handle,
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );

            let surface = unsafe {
                gl_config
                    .display()
                    .create_window_surface(&gl_config, &attrs)
                    .unwrap()
            };

            let gl_context = not_current_gl_context
                .take()
                .unwrap()
                .make_current(&surface)
                .unwrap();

            let renderer = unsafe {
                OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _)
            }
            .expect("Cannot create renderer");

            let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
            canvas.set_size(width, height, window.scale_factor() as f32);

            (canvas, window, gl_context, surface)
        };

        #[cfg(target_arch = "wasm32")]
        let (canvas, window) = {
            use wasm_bindgen::JsCast;

            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

            use winit::platform::web::WindowBuilderExtWebSys;

            let renderer = OpenGl::new_from_html_canvas(&canvas).expect("Cannot create renderer");

            let window = WindowBuilder::new()
                .with_canvas(Some(canvas))
                .build(&event_loop)
                .unwrap();

            let canvas = Canvas::new(renderer).expect("Cannot create canvas");

            (canvas, window)
        };

        self.run(
            canvas,
            event_loop,
            #[cfg(not(target_arch = "wasm32"))]
            context,
            #[cfg(not(target_arch = "wasm32"))]
            surface,
            window,
        );
    }

    fn run(
        &self,
        mut canvas: Canvas<OpenGl>,
        el: EventLoop<()>,
        #[cfg(not(target_arch = "wasm32"))] context: glutin::context::PossiblyCurrentContext,
        #[cfg(not(target_arch = "wasm32"))] surface: glutin::surface::Surface<
            glutin::surface::WindowSurface,
        >,
        window: Window,
    ) {
        let mut font_id = Vec::<FontId>::new();
        font_id.push(
            canvas
                .add_font_mem(include_bytes!("../math/osifont-lgpl3fe.ttf"))
                .expect("Cannot add font"),
        );

        //font_id.push(canvas
        //    .add_font_mem(&resource!("src/femtovg_plotter/assets/Roboto-Regular.ttf"))
        //    .expect("Cannot add font"));

        let mut screenshot_image_id = None;

        let mut mousex = 0.0;
        let mut mousey = 0.0;
        let mut dragging = false;

        let viewbox = if let Some(viewbox) = &self.viewbox {
            viewbox.clone()
        } else {
            todo!("no viebox set");
        };

        //let mut perf = PerfGraph::new();

        //let svg_data = include_str!("assets/Ghostscript_Tiger.svg").as_bytes();
        //let tree = usvg::Tree::from_data(svg_data, &usvg::Options::default()).unwrap();

        let paths = self.pathlist.clone();
        let texts = self.textlist.clone();
        // print memory usage
        let mut total_sisze_bytes = 0;

        for path in &self.pathlist {
            total_sisze_bytes += path.0.size();
        }

        log::info!("Path mem usage: {}kb", total_sisze_bytes / 1024);

        el.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            //println!("event: {:?}",event);
            match event {
                Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => match event {
                    #[cfg(not(target_arch = "wasm32"))]
                    WindowEvent::Resized(physical_size) => {
                        surface.resize(
                            &context,
                            physical_size.width.try_into().unwrap(),
                            physical_size.height.try_into().unwrap(),
                        );
                    }
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    } => match state {
                        ElementState::Pressed => dragging = true,
                        ElementState::Released => dragging = false,
                    },
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    } => {
                        if dragging {
                            let p0 = canvas
                                .transform()
                                .inversed()
                                .transform_point(mousex, mousey);
                            let p1 = canvas
                                .transform()
                                .inversed()
                                .transform_point(position.x as f32, position.y as f32);

                            canvas.translate(p1.0 - p0.0, p1.1 - p0.1);
                        }

                        mousex = position.x as f32;
                        mousey = position.y as f32;
                    }
                    WindowEvent::MouseWheel {
                        device_id,
                        delta: winit::event::MouseScrollDelta::PixelDelta(pos),
                        ..
                    } => {
                        let pt = canvas
                            .transform()
                            .inversed()
                            .transform_point(mousex, mousey);
                        canvas.translate(pt.0, pt.1);
                        canvas.scale(1.0 + (pos.y / 10.0) as f32, 1.0 + (pos.y / 10.0) as f32);
                        canvas.translate(-pt.0, -pt.1);
                    }
                    //WindowEvent::MouseWheel {
                    //    device_id: _,
                    //    delta: winit::event::MouseScrollDelta::LineDelta(x, y),
                    //    ..
                    //} => {
                    //    println!("== mouse wheel {x} {y}");
                    //    let pt = canvas.transform().inversed().transform_point(mousex, mousey);
                    //    canvas.translate(pt.0, pt.1);
                    //    canvas.scale(1.0 + (y / 10.0), 1.0 + (y / 10.0));
                    //    canvas.translate(-pt.0, -pt.1);
                    //}
                    WindowEvent::TouchpadRotate {
                        device_id,
                        delta,
                        phase,
                    } => {
                        println!("rotate {:?} {:?}", delta, phase);
                    }
                    WindowEvent::TouchpadMagnify {
                        device_id,
                        delta,
                        phase,
                    } => {
                        println!("magnify {:?} {:?}", delta, phase);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::S),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        if let Some(screenshot_image_id) = screenshot_image_id {
                            canvas.delete_image(screenshot_image_id);
                        }

                        if let Ok(image) = canvas.screenshot() {
                            screenshot_image_id = Some(
                                canvas
                                    .create_image(image.as_ref(), ImageFlags::empty())
                                    .unwrap(),
                            );
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    let dpi_factor = window.scale_factor();
                    let size = window.inner_size();

                    canvas.set_size(size.width, size.height, dpi_factor as f32);
                    canvas.clear_rect(0, 0, size.width, size.height, Color::rgbf(1.0, 1.0, 1.0));

                    canvas.save();

                    let f = size.width as f32 / viewbox.start.x;
                    canvas.scale(f, f);

                    for (path, fill, stroke) in &paths {
                        if let Some(fill) = fill {
                            canvas.fill_path(path, fill);
                        }

                        if let Some(stroke) = stroke {
                            canvas.stroke_path(path, stroke);
                        }

                        if canvas.contains_point(path, mousex, mousey, FillRule::NonZero) {
                            let mut paint = Paint::color(Color::rgb(32, 240, 32));
                            paint.set_line_width(1.0);
                            canvas.stroke_path(path, &paint);
                        }
                    }

                    for (x, y, text, paint) in &texts {
                        if let Some(paint) = paint {
                            canvas.fill_text(*x, *y, text, paint).unwrap();
                        }

                        //if canvas.contains_point(path, mousex, mousey, FillRule::NonZero) {
                        //    let mut paint = Paint::color(Color::rgb(32, 240, 32));
                        //    paint.set_line_width(1.0);
                        //    canvas.stroke_path(path, &paint);
                        //}
                    }

                    canvas.restore();

                    canvas.save();
                    canvas.reset();
                    //perf.render(&mut canvas, 5.0, 5.0);
                    canvas.restore();

                    canvas.flush();
                    #[cfg(not(target_arch = "wasm32"))]
                    surface.swap_buffers(&context).unwrap();
                }
                Event::MainEventsCleared => window.request_redraw(),
                _ => (),
            }
        });
    }
}
