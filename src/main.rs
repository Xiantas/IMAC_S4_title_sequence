#[allow(non_snake_case)]
mod geometry;


use image::RgbImage;

fn main() -> Result<(),()> {
    let mut img = RgbImage::new(1000, 1000);

    for x in 100..510 {
        for y in x..510 {
            img.put_pixel(x, y, image::Rgb([(x/2) as u8, (y/2) as u8, ((x+y)/4) as u8]));
        }
    }

    img.save("target/img.png").map_err(|_| ())?;

    Ok(())
}
