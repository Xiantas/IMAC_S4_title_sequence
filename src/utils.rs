use std::ops::AddAssign;

#[derive(PartialEq, Copy, Clone)]
pub struct Distrib<
    T: PartialOrd + Copy + AddAssign
> {
    start: T,
    end: T,
    step: T,
    current: T,
}

impl<T: PartialOrd + Copy + AddAssign> Distrib<T> {
    pub fn new(start: T, end: T, step: T) -> Self {
        Self {
            start,
            end,
            step,
            current: start,
        }
    }
}

impl<T> Iterator for Distrib<T>
where
    T: PartialOrd + Copy + AddAssign
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let val = self.current;
            self.current += self.step;

            Some(val)
        }
    }
}
