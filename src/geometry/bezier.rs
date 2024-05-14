// use image::RgbImage;

// use rayon::iter::ParallelIterator;

use super::basics::Point;
// use super::draw::Drawable;
// use crate::utils::Distrib;

#[derive(Copy, Clone, Debug)]
pub struct Bezier {
    pub start: Point,
    pub anchor1: Point,
    pub anchor2: Point,
    pub end: Point,
}

/*
impl Drawable for Bezier {
    fn draw(&self, image: &mut RgbImage) {
//        for (x, y, p) in 
        image.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
            let pixel_point = Point::new(x as f32, y as f32);
            for t in Distrib::<f32>::new(0.0, 1.0, 0.001) {
                let t2 = 1.0 - t;
                let bezier_point = t2 * (t2 * (t2 * self.start + t * self.anchor1) + t * (t2 * self.anchor1 + t * self.anchor2)) + t * (t2 * (t2 * self.anchor1 + t * self.anchor2) + t * (t2 * self.anchor2 + t * self.end));

                if (bezier_point - pixel_point).sq_norm() < width_interpolation(self.start_size, self.mid_size, self.end_size, t) {
                    *p = self.color;
                    continue
                }
            }
        });
    }
}

fn width_interpolation(start: f32, mid: f32, end: f32, t: f32) -> f32 {
    if t < 0.5 {
        start + 2.0*t * (mid - start)
    } else {
        mid + 2.0*(t-0.5) * (end - mid)
    }
}
*/
