use image::RgbImage;

pub trait Drawable {
    fn draw(&self, image: &mut RgbImage);
}
