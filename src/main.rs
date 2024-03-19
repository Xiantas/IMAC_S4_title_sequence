mod geometry;
mod utils;

use image::RgbImage;

use crate::geometry::{
    bezier::Bezier,
    draw::Drawable,
    Point,
};

fn main() -> Result<(),()> {
    let mut img = RgbImage::new(1000, 1000);

    let la_bez = Bezier {
        start: Point::new(300.0, 300.0),
        anchor1: Point::new(1000.0, 300.0),
        anchor2: Point::new(0.0, 700.0),
        end: Point::new(700.0, 700.0),

        start_size: 0.0,
        mid_size: 200.0,
        end_size: 0.0,

        color: image::Rgb([50, 205, 200]),
    };

    la_bez.draw(&mut img);

    img.save("target/img.png").map_err(|_| ())?;

    Ok(())
}
