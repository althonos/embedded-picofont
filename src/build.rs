// extern crate image;
extern crate lodepng;
extern crate bitvec;

use std::env::var;
use std::path::PathBuf;
use std::io::Write;
use lodepng::Image;
use lodepng::ffi::ColorType;


fn main() {
    let mut dst: bitvec::vec::BitVec = bitvec::vec::BitVec::new();
    let bmp = match lodepng::decode_file("src/font.png", ColorType::GREY, 8) {
        Ok(Image::Grey(bmp)) => bmp,
        Ok(_) => panic!("unexpected image format"),
        Err(e) => panic!("PNG loading error: {}", e),
    };

    for pixel in bmp.buffer.iter() {
        dst.push(pixel.0 != 0);
    }

    let filename = PathBuf::from(var("OUT_DIR").unwrap()).join("font.raw");
    std::fs::File::create(filename).unwrap().write_all(&dst.into_vec());
}
