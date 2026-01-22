const MASK: usize = 63;
const BN: usize = 64;
const BB: usize = 6;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet{
    data: Vec<usize>,
}

impl BitSet {
    #[inline]
    pub fn new(cap: usize) -> Self {
        BitSet{
            data: vec![0; (cap+MASK)>>BB],
        }
    }

    #[inline]
    pub fn build(base: Vec<usize>) -> Self {
        BitSet {
            data: base
        }
    }

    #[inline]
    pub fn set(&mut self, p: usize, f: bool){
        if f{
            self.data[p>>BB] |= 1<<(p&MASK);
        } else {
            self.data[p>>BB] &= !(1<<(p&MASK));
        }
    }

    #[inline]
    pub fn flip(&mut self, p: usize){
        self.data[p>>BB] ^= 1<<(p&MASK)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    pub fn get(&self, p: usize) -> bool {
        self.data[p>>BB] & (1 << (p&MASK)) != 0
    }

    #[inline]
    pub fn and(&self, rhs: &Self) -> Self {
        if self.len() < rhs.len() {
            let mut res = rhs.clone();
            for (i, &x) in self.data.iter().enumerate(){
                res.data[i] &= x;
            }
            res
        } else {
            let mut res = self.clone();
            for (i, &x) in rhs.data.iter().enumerate(){
                res.data[i] &= x;
            }
            res
        }
    }

    #[inline]
    pub fn or(&self, rhs: &Self) -> Self{
        if self.len() < rhs.len() {
            let mut res = rhs.clone();
            for (i, &x) in self.data.iter().enumerate(){
                res.data[i] |= x;
            }
            res
        } else {
            let mut res = self.clone();
            for (i, &x) in rhs.data.iter().enumerate(){
                res.data[i] |= x;
            }
            res
        }
    }

    #[inline]
    pub fn xor(&self, rhs: &Self) -> Self {
        if self.len() < rhs.len() {
            let mut res = rhs.clone();
            for (i, &x) in self.data.iter().enumerate(){
                res.data[i] ^= x;
            }
            res
        } else {
            let mut res = self.clone();
            for (i, &x) in rhs.data.iter().enumerate(){
                res.data[i] ^= x;
            }
            res
        }
    }

    // 配列上の左シフトです
    #[inline]
    pub fn get_shift_left(&self, k: usize) -> Self {
        let b = k>>BB;
        let r = k&MASK;
        let n = self.data.len();
        let mut res = vec![0; n];
        for i in 0..n.max(b)-b{
            res[i+b] |= self.data[i] << r;
            if r > 0 && i+b+1 < n {
                res[i+b+1] |= self.data[i] >> (BN-r);
            }
        }
        Self::build(res)
    }

    #[inline]
    pub fn get_shift_right(&self, k: usize) -> Self {
        let b = k>>BB;
        let r = k&MASK;
        let n = self.data.len();
        let mut res = vec![0; n];
        for i in b..n{
            res[i-b] |= self.data[i] >> r;
            if r > 0 && b+1 <= i {
                res[i-b-1] |= self.data[i] << (BN-r);
            }
        }
        Self::build(res)
    }

    #[inline]
    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum::<usize>()
    }

    #[inline]
    pub fn count_zeros(&self) -> usize {
        self.data.iter().map(|x| x.count_zeros() as usize).sum::<usize>()
    }
}
