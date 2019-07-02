#![no_std]

use embedded_graphics::fonts::font_builder::FontBuilder;
use embedded_graphics::fonts::font_builder::FontBuilderConf;

#[derive(Debug, Copy, Clone)]
pub struct FontPicoConf;

impl FontBuilderConf for FontPicoConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/font.raw"));
    const CHAR_HEIGHT: u32 = 6;
    const CHAR_WIDTH: u32 = 4;
    const FONT_IMAGE_WIDTH: u32 = 128;
    fn char_offset(c: char) -> u32 {
        if c <= '\u{7f}' {
            (c as u32) * 2
        } else if c <= '\u{b3}' {
            c as u32 + 0x80
        } else {
            ('?' as u32) * 2
        }
    }
}

pub type FontPico<'a, C> = FontBuilder<'a, C, FontPicoConf>;

#[macro_export]
/// Render text using the [`FontPico`] font
///
/// [`FontPico`]:
macro_rules! text_pico {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use embedded_graphics::style::WithStyle;
        $crate::FontPico::render_str($text)
            $( .$style_key($style_value) )*
    }};
}
