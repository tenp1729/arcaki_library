pub struct BIT<T> where T:Copy+std::ops::Add<Output=T>+std::ops::Sub<Output=T>{
    n: usize,
    vec: Vec<T>,
    zero: T,
}

impl<T> BIT<T> where T:Copy+std::ops::Add<Output=T>+std::ops::Sub<Output=T>{
    pub fn new(n: usize, zero: T)->Self{
        let k = n.next_power_of_two();
        let base = vec![zero; k+2];
        BIT{n: k, vec: base, zero}
    }

    pub fn add(&mut self, mut idx: usize, x: T){
        idx += 1;
        while idx <= self.n{
            self.vec[idx] = self.vec[idx]+x;
            idx += idx&(!idx+1);
        }
    }

    pub fn g(&mut self, mut r: usize)->T{
        let mut res = self.zero;
        while r > 0{
            res = res+self.vec[r];
            r -= r&(!r+1);
        }
        res
    }

    // [l, r)で作動
    pub fn get(&mut self, l: usize, r: usize)->T{
        self.g(r)-self.g(l)
    }
}
