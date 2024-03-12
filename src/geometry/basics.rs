#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point{x, y}
    }

    pub fn translate(&mut self, p2: Point) {
        self.x += p2.x;
        self.y += p2.y;
    }
}

fn truc() {
    let mut x = Point::new(0.0, 0.1);
    let y = Point {x: 0.2, y: 0.3};

    x.translate(y);
}
