use crate::geometry::{
    bezier::Bezier,
    Point,
};

use random::Source;

pub fn drawings() -> Vec<Vec<Bezier>> {
    let mut chaos = random::default(666);


    let mut res = Vec::new();
    res.resize_with(10, || {
        let mut in_res = Vec::new();
        in_res.resize_with(1, || Bezier {
            start: Point::new(eee(&mut chaos), eee(&mut chaos)),
            anchor1: Point::new(eee(&mut chaos), eee(&mut chaos)),
            anchor2: Point::new(eee(&mut chaos), eee(&mut chaos)),
            end: Point::new(eee(&mut chaos), eee(&mut chaos)),
        });
        in_res
    });
    println!("Des seins : {:?}", &res);
    res
}

fn eee<T: Source>(chaos: &mut T) -> f32 {
    chaos.read::<f32>()*2.0 - 1.0
}
