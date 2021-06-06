extern crate embedded_graphics;
extern crate embedded_picofont;
extern crate lodepng;

use std::path::Path;
use std::path::PathBuf;

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Gray8;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_picofont::PICO_FONT;
use lodepng::ffi::ColorType;
use lodepng::Bitmap;
use lodepng::Grey;
use lodepng::Image;

pub struct GreyDisplay(Bitmap<Grey<u8>>);

impl DrawTarget for GreyDisplay {
    type Color = Gray8;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if (coord[0] as usize) < self.0.width && (coord[1] as usize) < self.0.height {
                let index = coord[0] as usize + coord[1] as usize * self.0.width;
                self.0.buffer[index] = Grey(color.luma());
            }
        }

        Ok(())
    }
}

impl OriginDimensions for GreyDisplay {
    fn size(&self) -> Size {
        Size::new(self.0.width as u32, self.0.height as u32)
    }
}

fn test_grayscale(reference: &Path, text: &str) {
    let exp = match lodepng::decode_file(reference, ColorType::GREY, 8) {
        Ok(Image::Grey(bmp)) => bmp,
        Ok(_other) => panic!("invalid PNG format"),
        Err(e) => panic!("could not load reference image: {}", e),
    };

    let mut display: GreyDisplay = GreyDisplay(Bitmap {
        buffer: vec![Grey(0); exp.width * exp.height],
        width: exp.width,
        height: exp.height,
    });

    Text::new(
        text,
        Point::new(0, 6),
        MonoTextStyle::new(&PICO_FONT, Gray8::WHITE),
    )
    .draw(&mut display)
    .unwrap();

    if exp.buffer != display.0.buffer {
        lodepng::encode_file(
            "/tmp/expected.png",
            &exp.buffer,
            exp.width,
            exp.height,
            ColorType::GREY,
            8,
        )
        .unwrap();
        lodepng::encode_file(
            "/tmp/actual.png",
            &display.0.buffer,
            display.0.width,
            display.0.height,
            ColorType::GREY,
            8,
        )
        .unwrap();
        panic!("images don't match: see /tmp");
    }
}

#[test]
fn test_hello_world() {
    test_grayscale(&PathBuf::from("tests/helloworld.png"), "Hello, world!");
}

#[test]
fn test_abcdefg() {
    test_grayscale(&PathBuf::from("tests/abcdefg.png"), "a.b,c:d;e'f!g");
}

#[test]
fn test_buttons() {
    test_grayscale(
        &PathBuf::from("tests/buttons.png"),
        "\u{96}\u{97}\u{86}\u{87}\u{a2}\u{a3}\u{a8}\u{a9}\u{9c}\u{9d}\u{ae}\u{af}",
    );
}

#[test]
fn test_unicode() {
    test_grayscale(&PathBuf::from("tests/unicode.png"), "😀");
}
