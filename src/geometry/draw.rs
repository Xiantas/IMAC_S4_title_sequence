use image::{Rgb, RgbImage};

pub trait Drawable {
    fn draw(&self, image: &mut RgbImage);
}
