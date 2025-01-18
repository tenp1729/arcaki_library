pub trait SegTreeMonoid{
    type S: Clone;
    fn identity() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct SegTree<M: SegTreeMonoid> {
    n: usize,
    data: Vec<M::S>,
}

impl<M: SegTreeMonoid> SegTree<M> {
    pub fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        let data = vec![M::identity(); 2*n];
        SegTree{ n, data }
    }

    pub fn set(&mut self, i: usize, x: M::S) {
        let mut p = i + self.n;
        self.data[p] = x;
        while p > 0 {
            p /= 2;
            self.data[p] = M::op(&self.data[p << 1], &self.data[(p << 1) | 1]);
        }
    }

    pub fn push(&mut self, i: usize, x: M::S) {
        let mut p = i + self.n;
        self.data[p] = M::op(&self.data[p], &x);
        while p > 0 {
            p /= 2;
            self.data[p] = M::op(&self.data[p << 1], &self.data[(p << 1) | 1]);
        }
    }

    pub fn prod(&mut self, l: usize, r: usize) -> M::S {
        let mut p_l = l + self.n;
        let mut p_r = r + self.n;
        let mut res_l = M::identity();
        let mut res_r = M::identity();
        while p_l < p_r {
            if p_l & 1 == 1 {
                res_l = M::op(&res_l, &self.data[p_l]);
                p_l += 1;
            }
            if p_r & 1 == 1 {
                p_r -= 1;
                res_r = M::op(&self.data[p_r], &res_r);
            }
            p_l >>= 1;
            p_r >>= 1;
        }
        M::op(&res_l, &res_r)
    }

    pub fn all_prod(&mut self)-> M::S {
        self.data[1].clone()
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize where F: Fn(&M::S)->bool {
        assert!(f(&M::identity())); // これはバグってくれないと多分デバックが悲惨
        if l == self.n {
            return self.n 
        }
        l += self.n; 
        let mut ac = M::identity();
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::op(&ac, &self.data[l])) {
                while l < self.n {
                    l <<= 1;
                    let res = M::op(&ac, &self.data[l]);
                    if f(&res) {
                        ac = res;
                        l += 1;
                    }
                }
                return l - self.n;
            }
            ac = M::op(&ac, &self.data[l]);
            l += 1;
            let z = l as isize;
            (z & -z) != z
        } {}
        self.n
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize where F: Fn(&M::S) -> bool {
        assert!(f(&M::identity()));
        if r == 0 {return 0}
        r += self.n;
        let mut ac = M::identity();
        while {
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::op(&ac, &self.data[r])) {
                while r < self.n{
                    r = 2 * r + 1;
                    let res = M::op(&ac, &self.data[r]);
                    if f(&res) {
                        ac = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }
            ac = M::op(&ac, &self.data[r]);
            let z = r as isize;
            z & -z == z
        } {}
        0
    }
}

struct SegTreeMax;
impl SegTreeMonoid for SegTreeMax {
    type S = i64;
    fn identity() -> Self::S {
        i64::MIN
    }

    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
}

struct SegTreeMin;
impl SegTreeMonoid for SegTreeMin {
    type S = i64;
    fn identity() -> Self::S {
        i64::MAX
    }

    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.min(b)
    }
}

struct SegTreeSum;
impl SegTreeMonoid for SegTreeSum {
    type S = i64;
    fn identity() -> Self::S {
        0
    }

    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }
}

struct SegTreeXor;
impl SegTreeMonoid for SegTreeXor {
    type S = i64;
    fn identity() -> Self::S {
        0
    }

    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a ^ b
    }
}

pub const B: u128 = 127;
pub const MO: u128 = (1<<64)-59;

pub struct RollingHashMonoid;
impl SegTreeMonoid for RollingHashMonoid {
    type S = (u128, u128);

    fn identity() -> Self::S {
        (1, 0)
    }

    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        let &(b1, h1) = a;
        let &(b2, h2) = b;
        ((b1 * b2) % MO, (b2 * h1 + h2) % MO)
    }
}

pub fn compress<T: Copy+Ord+std::hash::Hash>(a: &Vec<T>) -> Vec<usize> {
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut dic = std::collections::HashMap::new();
    for (i, &v) in b.iter().enumerate() {
        dic.insert(v, i);
    }
    a.iter().map(|x| dic[x]).collect()
}
