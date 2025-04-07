#[derive(Copy, Clone, Debug)]
pub struct Ratio{
    x: i64,
    y: i64,
}

impl Ratio {
    pub fn new(mut x: i64, mut y: i64) -> Self {
        if x < 0{
            (x, y) = (-x, -y);
        }
        let g = gcd(x, y).abs();
        Ratio {x: x/g, y:y/g}
    }

    pub fn int(x: i64)->Self{
        Ratio{x:1, y: x}
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

fn line_equator(u: (i64, i64), v: (i64, i64)) -> (i64, i64, i64){
    (v.1-u.1, u.0-v.0, u.0*v.1-u.1*v.0)
}

fn cross_point(l1: (i64, i64, i64), l2: (i64, i64, i64)) -> (Ratio, Ratio){
    let (a1, b1, c1) = l1;
    let (a2, b2, c2) = l2;
    if a1*b2==a2*b1{
        (Ratio::new(1, INF), Ratio::new(1, INF))
    } else {
        let y = Ratio::new(a1*b2-a2*b1, a1*c2-c1*a2);
        let x = Ratio::new(a1*b2-a2*b1, c1*b2-b1*c2);
        (x, y)
    }
}
