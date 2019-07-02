//! *The PICO-8 font to use with [`embedded-graphics`](https://docs.rs/embedded-graphics).*
//!
//! [![TravisCI](https://img.shields.io/travis/althonos/embedded-picofont/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/embedded-picofont/branches)
//! [![Codecov](https://img.shields.io/codecov/c/gh/althonos/embedded-picofont/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/embedded-picofont)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/embedded-picofont)
//! [![Crate](https://img.shields.io/crates/v/embedded-picofont.svg?maxAge=600&style=flat-square)](https://crates.io/crates/embedded-picofont)
//! [![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/embedded-picofont)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/embedded-picofont/blob/master/CHANGELOG.md)
//! [![GitHub issues](https://img.shields.io/github/issues/althonos/embedded-picofont.svg?style=flat-square)](https://github.com/althonos/embedded-picofont/issues)
//!
//! # Overview
//!
//! This crate provides the super tiny 4x6 font of the PICO-8 fantasy console:
//!
//! ![Font](https://www.lexaloffle.com/gfx/pico8_font.png)
//!
//! Please note the PICO-8 itself only use the uppercase characters, as the lowercase
//! chars can get *really* hard to read if the display is not upscaled. As such, it
//! is advised to only use this font to display uppercase characters.
//!
//! # Usage
//!
//! Use the [`text_pico`] macro to create a `Drawable` text from a string:
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_picofont::{text_pico, FontPico};
//! let text: FontPico<u8> = text_pico!("Hello world!");
//! ```
//! ![Hello world](https://github.com/althonos/embedded-picofont/raw/master/static/helloworld.png)
//!
//! The PICO-8 also has wide characters: these can be displayed using two smaller
//! characters in the `128..255` char range:
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_picofont::{text_pico, FontPico};
//! let text: FontPico<u8> = text_pico!("PRESS \u{96}\u{97} TO GO BACK");
//! ```
//! ![Press left to go back](https://github.com/althonos/embedded-picofont/raw/master/static/goback.png)
//!
//! See the [`embedded-graphics` documentation](https://docs.rs/embedded-graphics/)
//! for more information on how to use this crate.
//!
//! # License
//!
//! * The [original PICO-8 font](https://www.lexaloffle.com/pico-8.php?page=faq)
//!   is available under the [CC-0 license](https://creativecommons.org/share-your-work/public-domain/cc0/).
//! * This crate is released under the [MIT License](https://opensource.org/licenses/mit-license.php).
//!

#![no_std]

use embedded_graphics::fonts::font_builder::FontBuilder;
use embedded_graphics::fonts::font_builder::FontBuilderConf;

#[derive(Debug, Copy, Clone)]
/// The [`FontBuilderConf`] implemtation for the PICO-8 font.
///
/// [`FontBuilderConf`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/fonts/font_builder/trait.FontBuilderConf.html
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

/// The 4x6 pixel monospace font used by the PICO-fantasy 8 console.
///
/// # Examples
///
/// ## Write some text to the screen at the default `(0, 0)` position
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_picofont::FontPico;
/// use embedded_picofont::text_pico;
/// # use embedded_graphics::mock_display::Display;
/// # let mut display = Display::default();
///
/// // Use struct methods directly
/// display.draw(FontPico::render_str("Hello Rust!"));
///
/// // Use a macro instead
/// display.draw(text_pico!("Hello Rust!"));
/// ```
///
/// ## Translate text by (20px, 30px)
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_picofont::FontPico;
/// # use embedded_graphics::mock_display::Display;
/// # let mut display = Display::default();
///
/// display.draw(
///     FontPico::render_str("Hello Rust!").translate(Coord::new(20, 30))
/// );
/// ```
///
/// ## Add some styling to the text
///
/// Use [any method provided by the `WithStyle` trait](https://docs.rs/embedded-graphics/latest/embedded_graphics/style/trait.WithStyle.html#required-methods).
/// Properties like `fill` or `stroke` passed to the `text_pico` macro are converted into method
/// calls verbatim.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_picofont::text_pico;
/// use embedded_picofont::FontPico;
/// # use embedded_graphics::mock_display::Display;
/// # let mut display = Display::default();
///
/// display.draw(text_pico!(
///     "Hello Rust!",
///     fill = Some(1u8),
///     stroke = Some(0u8)
/// ));
///
/// display.draw(
///     FontPico::render_str("Hello Rust!")
///         .translate(Coord::new(20, 30))
///         .fill(Some(1u8))
///         .stroke(Some(0u8)),
/// );
/// ```
pub type FontPico<'a, C> = FontBuilder<'a, C, FontPicoConf>;

#[macro_export]
/// Render text using the [`FontPico`] font.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_picofont::{text_pico, FontPico};
///
/// let text: FontPico<u8> = text_pico!("Hello world!");
/// let styled_text: FontPico<u8> = text_pico!(
///     "Hello world!",
///     stroke = Some(10u8),
/// );
/// ```
///
/// Style properties like `stroke` map to the method calls on the [`WithStyle`].
/// trait.
///
/// [`FontPico`]: ./type.FontPico.html
/// [`WithStyle`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/style/trait.WithStyle.html
macro_rules! text_pico {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use embedded_graphics::style::WithStyle;
        $crate::FontPico::render_str($text)
            $( .$style_key($style_value) )*
    }};
}
