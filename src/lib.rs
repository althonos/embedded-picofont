//! *The PICO-8 font to use with [`embedded-graphics`](https://crates.io/crates/embedded-graphics).*
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
//! This crate provides the super tiny 4x6 font of the PICO-8 fantasy console as an [`embedded_graphics::fonts::Font`](https://docs.rs/embedded-graphics/0.6.2/embedded_graphics/fonts/trait.Font.html)::
//!
//! <img src="https://www.lexaloffle.com/gfx/pico8_font.png" style="width:100%;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: crisp-edges;-ms-interpolation-mode: nearest-neighbor;">
//!
//! Please note the PICO-8 itself only use the uppercase characters, as the lowercase
//! chars can get *really* hard to read if the display is not upscaled. As such, it
//! is advised to only use this font to display uppercase characters.
//!
//! # Usage
//!
//! Use [`TextStyle`](https://docs.rs/embedded-graphics/0.6.2/embedded_graphics/style/struct.TextStyle.html)
//! to attach the PICO-8 font to a text:
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::Text;
//! use embedded_graphics::pixelcolor::Gray8;
//! use embedded_graphics::style::TextStyle;
//! use embedded_picofont::FontPico;
//!
//! let text = Text::new("Hello world!", Point::new(0, 0))
//!     .into_styled(TextStyle::new(FontPico, Gray8::WHITE));
//! ```
//! ![Hello world](https://github.com/althonos/embedded-picofont/raw/master/static/helloworld.png)
//!
//! The PICO-8 also has wide characters: these can be displayed using two smaller
//! characters in the `128..255` char range:
//! ```rust
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics::fonts::Text;
//! # use embedded_graphics::pixelcolor::Gray8;
//! # use embedded_graphics::style::TextStyle;
//! # use embedded_picofont::FontPico;
//!
//! let text = Text::new("PRESS \u{96}\u{97} TO GO BACK", Point::new(0, 0))
//!     .into_styled(TextStyle::new(FontPico, Gray8::WHITE));
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

use embedded_graphics::fonts::Font;
use embedded_graphics::geometry::Size;

/// The 4x6 pixel monospace font used by the PICO-fantasy 8 console.
///
/// # Examples
///
/// ## Write some text to the screen at the default `(0, 0)` position
///
/// ```rust
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::fonts::Text;
/// # use embedded_graphics::pixelcolor::Gray8;
/// # use embedded_graphics::style::TextStyle;
/// # use embedded_picofont::FontPico;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// Text::new("Hello Rust!", Point::new(0, 0))
///     .into_styled(TextStyle::new(FontPico, Gray8::WHITE))
///     .draw(&mut display);
/// ```
///
/// ## Translate text by (20px, 30px)
///
/// ```rust
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::fonts::Text;
/// # use embedded_graphics::pixelcolor::Gray8;
/// # use embedded_graphics::style::TextStyle;
/// # use embedded_picofont::FontPico;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// Text::new("Hello Rust!", Point::new(20, 30))
///     .into_styled(TextStyle::new(FontPico, Gray8::WHITE))
///     .draw(&mut display);
/// ```
///
/// ## Add some styling to the text
///
/// Use the [`TextStyleBuilder`](https://docs.rs/embedded-graphics/0.6.1/embedded_graphics/style/struct.TextStyleBuilder.html)
/// trait to edit the colors of the rendered text:
///
/// ```rust
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::fonts::Text;
/// # use embedded_graphics::pixelcolor::Rgb888;
/// # use embedded_graphics::style::TextStyleBuilder;
/// # use embedded_picofont::FontPico;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// let style = TextStyleBuilder::new(FontPico)
///     .text_color(Rgb888::MAGENTA)
///     .background_color(Rgb888::BLACK)
///     .build();
///
/// Text::new("Hello Rust!", Point::new(0, 0))
///     .into_styled(style)
///     .draw(&mut display);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FontPico;

impl Font for FontPico {
    const FONT_IMAGE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/font.raw"));
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(4, 6);

    fn char_offset(c: char) -> u32 {
        if c <= '\u{7f}' {
            return (c as u32) * 2;
        } else if c <= '\u{b3}' {
            return c as u32 + 0x80;
        }
        ('?' as u32) * 2
    }
}
