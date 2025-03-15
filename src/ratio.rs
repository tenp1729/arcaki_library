use std::cmp::{Ord, Ordering};

#[derive(Copy, Clone, Debug)]
pub struct Ratio{
    x: i64,
    y: i64,
}

impl Ratio {
    pub fn new(x: i64, y: i64) -> Self {
        Ratio {x, y}
    }
}

impl PartialEq<Self> for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.y*other.x == self.x*other.y
    }
}

impl PartialOrd<Self> for Ratio{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x1 = self.y*other.x;
        let y1 = self.x*other.y;
        x1.partial_cmp(&y1)
    }
}

impl Eq for Ratio {}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
