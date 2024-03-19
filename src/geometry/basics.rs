use std::ops::{
    Add,
    Mul,
    Sub,
};

#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point{x, y}
    }

    pub fn sq_norm(&self) -> f32 {
        self.x*self.x + self.y*self.y
    }

    pub fn norm(&self) -> f32 {
        self.sq_norm().sqrt()
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        Point::new(self * p.x, self * p.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, p: Point) -> Self::Output {
        Point::new(self.x + p.x, self.y + p.y)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, p: Point) -> Self::Output {
        Point::new(self.x - p.x, self.y - p.y)
    }
}
