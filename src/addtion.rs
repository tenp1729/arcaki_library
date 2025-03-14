pub fn get_sqrt(x: i64) -> i64 {
    let mut res = (x as f64).sqrt() as i64 - 100;
    while res < 0 || (res + 1) * (res + 1) <= x{
        res += 1;
    }
    res
}

use std::ops::{Add, Sub};

pub struct Cumsum2D<T> where T: Copy+Add<Output=T>+Sub<Output=T>+Default{
    ac: Vec<Vec<T>>,
}

impl<T> Cumsum2D<T> where T: Copy+Add<Output=T>+Sub<Output=T>+Default{
    pub fn new(data: &Vec<Vec<T>>)->Self where <T as Add>::Output: Add<T>{
        let h = data.len();
        let w = data[0].len();
        let mut ac = vec![vec![T::default(); w+1]; h+1];
        for i in 0..h{
            for j in 0..w{
                ac[i+1][j+1] = data[i][j]+ac[i+1][j]+ac[i][j+1]-ac[i][j];
            }
        }
        Cumsum2D{
            ac
        }
    }

    pub fn query(&self, lx: usize, ly: usize, rx: usize, ry: usize)->T{
        let ac = &self.ac;
        ac[rx][ry]+ac[lx][ly]-ac[lx][ry]-ac[rx][ly]
    }
}

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Counter<T>{
    map: BTreeMap<T, usize>,
}

impl<T: Copy+Ord> Counter<T>{
    pub fn new()->Self{
        Counter{
            map: BTreeMap::new(),
        }
    }

    #[inline(always)]
    pub fn mi(&self)->T{
        *self.map.range(..).next().unwrap().0
    }

    #[inline(always)]
    pub fn mx(&self)->T{
        *self.map.range(..).next_back().unwrap().0
    }

    #[inline(always)]
    pub fn one_add(&mut self, x: T){
        *self.map.entry(x).or_insert(0) += 1;
    }

    #[inline(always)]
    pub fn one_sub(&mut self, x: T){
        let e = self.map.entry(x).or_insert(0);
        *e = e.saturating_sub(1);
        if self.map[&x] <= 0{
            self.map.remove(&x);
        }
    }

    #[inline(always)]
    pub fn one_update(&mut self, x: T, y: T){
        self.one_sub(x);
        self.one_add(y);
    }

    #[inline(always)]
    pub fn del(&mut self, x: T){
        self.map.remove(&x);
    }

    #[inline(always)]
    pub fn add(&mut self, x: T, c: usize){
        *self.map.entry(x).or_insert(0) += c;
    }

    #[inline(always)]
    pub fn sub(&mut self, x: T, c: usize){
        let e = self.map.entry(x).or_insert(0);
        *e = e.saturating_sub(c);
        if self.map[&x] == 0{
            self.map.remove(&x);
        }
    }

    #[inline(always)]
    pub fn include(&self, x: T)->bool{
        self.map.contains_key(&x)
    }

    #[inline(always)]
    pub fn cnt(&self, x: T)->usize{
        *self.map.get(&x).unwrap_or(&0)
    }

    #[inline(always)]
    pub fn is_empty(&self)->bool{
        self.map.is_empty()
    }

    #[inline(always)]
    pub fn len(&self)->usize{
        self.map.len()
    }
}
