// extern crate image;
extern crate lodepng;
extern crate bitvec;

use std::env::var;
use std::path::PathBuf;
use std::io::Write;
use lodepng::Image;
use lodepng::ffi::ColorType;

fn main() {
    // Load the original image.
    let mut dst: bitvec::vec::BitVec = bitvec::vec::BitVec::new();
    let bmp = match lodepng::decode_file("src/font.png", ColorType::GREY, 8) {
        Ok(Image::Grey(bmp)) => bmp,
        Ok(_) => panic!("unexpected image format"),
        Err(e) => panic!("PNG loading error: {}", e),
    };

    // Create a bitmap of depth 1
    for pixel in bmp.buffer.iter() {
        dst.push(pixel.0 != 0);
    }

    // Write the generated bitmap.
    let filename = PathBuf::from(var("OUT_DIR").unwrap()).join("font.raw");
    std::fs::File::create(filename)
        .and_then(|mut f| f.write_all(&dst.into_vec()))
        .expect("could ");
}
