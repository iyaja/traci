// #[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate image;

// use na::{Vector3, Rotation3, Matrix4};
use image::{ImageBuffer, RgbImage, Rgb};


fn main() {

    let mut img: RgbImage = ImageBuffer::new(512, 512);

    for (x, y, pix) in img.enumerate_pixels_mut() {
        let r = (x * 255 / 512) as u8;
        let g = (y * 255 / 512) as u8;
        *pix = Rgb([r, g, 64]);
    }
    
    img.save("test.png").unwrap();

}