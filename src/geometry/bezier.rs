use super::basics::Point;

pub struct Bezier {
    start: Point,
    end: Point,
    anchor1: Point,
    anchor2: Point,
    start_size: f32,
    mid_size: f32,
    end_size: f32,
}
