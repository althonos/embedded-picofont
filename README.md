# `embedded-picofont` [![Star me](https://img.shields.io/github/stars/althonos/embedded-picofont.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/embedded-picofont/stargazers)

*The PICO-8 font to use with [`embedded-graphics`](https://docs.rs/embedded-graphics).*

[![TravisCI](https://img.shields.io/travis/althonos/embedded-picofont/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/embedded-picofont/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/embedded-picofont/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/embedded-picofont)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/embedded-picofont)
[![Crate](https://img.shields.io/crates/v/embedded-picofont.svg?maxAge=600&style=flat-square)](https://crates.io/crates/embedded-picofont)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/embedded-picofont)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/embedded-picofont/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/althonos/embedded-picofont.svg?style=flat-square)](https://github.com/althonos/embedded-picofont/issues)

## Overview

This crate provides the super tiny 4x6 font of the PICO-8 fantasy console as
an [`embedded_graphics::fonts::Font`](https://docs.rs/embedded-graphics/0.5.1/embedded_graphics/fonts/trait.Font.html):

![Font](https://www.lexaloffle.com/gfx/pico8_font.png)

Please note the PICO-8 itself only use the uppercase characters, as the lowercase
chars can get *really* hard to read if the display is not upscaled. As such, it
is advised to only use this font to display uppercase characters.

## Usage

Use the `text_pico` to create a `Drawable` text from a string:
```rust
use embedded_picofont::{text_pico, FontPico};
let text: FontPico<u8> = text_pico!("Hello world!");
```
![Hello world](https://github.com/althonos/embedded-picofont/raw/master/static/helloworld.png)

The PICO-8 also has wide characters: these can be displayed using two smaller
characters in the `128..255` char range:
```rust
use embedded_picofont::{text_pico, FontPico};
let text: FontPico<u8> = text_pico!("PRESS \u{96}\u{97} TO GO BACK");
```
![Press left to go back](https://github.com/althonos/embedded-picofont/raw/master/static/goback.png)

See the [`embedded-graphics` documentation](https://docs.rs/embedded-graphics/)
for more information on how to use this crate.

## License

* The [original PICO-8 font](https://www.lexaloffle.com/pico-8.php?page=faq)
  is available under the [CC-0 license](https://creativecommons.org/share-your-work/public-domain/cc0/).
* This crate is released under the [MIT License](https://opensource.org/licenses/mit-license.php).
