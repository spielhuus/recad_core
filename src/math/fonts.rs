use std::{collections::HashMap, fs::File, io::Read, sync::Mutex};

use fontdue::{
    layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign},
    Font,
};
use lazy_static::lazy_static;
use ndarray::{arr2, Array2};
use rust_fontconfig::{FcFontCache, FcPattern};

use crate::{
    gr::{self, Color, Effects, Justify, Pos},
    Error,
};

pub const SCALE: f32 = 25.4 * 0.72; //TODO defined multiple times
pub static OSIFONT: &[u8] = include_bytes!("osifont-lgpl3fe.ttf");
lazy_static! {
    static ref FONT_CACHE: FcFontCache = FcFontCache::build();
    static ref FONTS: Mutex<HashMap<String, Font>> = Mutex::new(HashMap::new());
}

#[inline(always)]
fn get_face(effects: &Effects) -> String {
    if let Some(face) = &effects.font.face {
        face.to_string()
    } else {
        String::from("osifont")
    }
}

pub fn load_font(face: &str) -> Result<(), Error> {
    let mut last = FONTS.lock().unwrap();
    if !last.contains_key(face) {
        let font = if face == "osifont" {
            OSIFONT.to_vec()
        } else {
            let Some(result) = FONT_CACHE.query(&FcPattern {
                name: Some(face.to_string()),
                ..Default::default()
            }) else {
                return Err(Error(
                    String::from("font-error"),
                    format!("Unable to load font: {face}"),
                ));
            };

            let result = result.path.to_string();
            let Ok(mut f) = File::open(result) else {
                return Err(Error(
                    String::from("font-error"),
                    format!("Unable to load font: {face}"),
                ));
            };

            let mut font = Vec::new();
            f.read_to_end(&mut font)?;
            font
        };

        last.insert(
            face.to_string(),
            Font::from_bytes(font, fontdue::FontSettings::default()).unwrap(),
        );
    }
    Ok(())
}

pub fn dimension(text: &str, effects: &gr::Effects) -> Result<Array2<f32>, Error> {
    let face = get_face(effects);
    load_font(&face)?;
    let last = FONTS.lock().unwrap();
    let fonts = &[last.get(&face).unwrap()];

    let mut layout = Layout::new(CoordinateSystem::PositiveYUp);
    layout.reset(&LayoutSettings {
        ..LayoutSettings::default()
    });
    layout.append(
        fonts,
        &TextStyle::new(text, effects.font.size.0 * 1.333333, 0),
    );
    let width: usize = layout.glyphs().iter().map(|g| g.width).sum();

    Ok(arr2(&[[
        width as f32,
        effects.font.size.0 * 1.333333, //TODO: get height from layout?
    ]]))
}

pub fn anchor(effects: &Effects) -> HorizontalAlign {
    if effects.justify.contains(&Justify::Right) {
        HorizontalAlign::Right
    } else if effects.justify.contains(&Justify::Left) {
        HorizontalAlign::Left
    } else {
        HorizontalAlign::Center
    }
}

pub fn baseline(effects: &Effects) -> VerticalAlign {
    if effects.justify.contains(&Justify::Bottom) {
        VerticalAlign::Bottom
    } else if effects.justify.contains(&Justify::Top) {
        VerticalAlign::Top
    } else {
        VerticalAlign::Middle
    }
}

pub struct GlyphItem {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn rasterize(text: &str, pos: &Pos, effects: &gr::Effects) -> Result<Vec<GlyphItem>, Error> {
    let face = get_face(effects);
    load_font(&face)?;
    let last = FONTS.lock().unwrap();
    let font = last.get(&face).unwrap();

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x: pos.x,
        y: pos.y,
        horizontal_align: anchor(effects),
        vertical_align: baseline(effects),
        ..Default::default()
    });
    layout.append(
        &[font],
        &TextStyle::new(text, effects.font.size.0 * 1.3333 * SCALE, 0),
    );

    let (r, g, b) = match effects.font.color.unwrap() {
        Color::None => (255, 255, 255),
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Rgba(r, g, b, _) => (r, g, b),
    };

    let mut result_glyphs = vec![];
    for glyph in layout.glyphs() {
        let mut buffer = vec![];
        let (metrics, coverage) = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px);
        for lightness in coverage.into_iter() {
            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
            buffer.push(lightness);
        }
        if metrics.width > 0 {
            result_glyphs.push(GlyphItem {
                x: glyph.x as u32,
                y: glyph.y as u32,
                width: glyph.width as u32,
                height: glyph.height as u32,
                data: buffer,
            });
        }
    }
    Ok(result_glyphs)
}
