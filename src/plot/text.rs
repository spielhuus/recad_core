use fontdue::{Font, FontResult};

use crate::math::fonts::OSIFONT;




pub fn load_font() -> FontResult<Font> {
    let settings = fontdue::FontSettings::default();
    fontdue::Font::from_bytes(OSIFONT.to_vec(), settings)
}

pub fn rgb_to_u32(red: usize, green: usize, blue: usize, alpha: usize) -> u32 {
    let r = red.clamp(0, 255);
    let g = green.clamp(0, 255);
    let b = blue.clamp(0, 255);
    let a = alpha.clamp(0, 255);
    ((a << 24) | (r << 16) | (g << 8) | b) as u32
}



