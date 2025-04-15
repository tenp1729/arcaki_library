use std::{mem::swap, collections::HashMap};

pub struct RollbackUnionFind{
    parent: Vec<i32>,
    hist: Vec<(usize, usize, i32, i32)>,
}

impl RollbackUnionFind{
    pub fn new(n: usize)->Self{
        RollbackUnionFind{
            parent: vec![-1; n],
            hist: Vec::new(),
        }
    }

    pub fn find(&self, u: usize)->usize{
        let mut res = u;
        while self.parent[res] >= 0{
            res = self.parent[res] as usize;
        }
        res
    }

    pub fn merge(&mut self, u: usize, v: usize)->bool{
        let (mut pu, mut pv) = (self.find(u), self.find(v));
        if pu==pv{
            self.hist.push((!0, !0, -1, -1));
            return false;
        }
        if self.parent[pu] > self.parent[pv]{
            swap(&mut pu, &mut pv);
        }
        self.hist.push((pu, pv, self.parent[pu], self.parent[pv]));
        self.parent[pu] += self.parent[pv];
        self.parent[pv] = pu as i32;
        true
    }

    pub fn same(&self, u: usize, v: usize) -> bool{
        self.find(u)==self.find(v)
    }

    pub fn size(&mut self, p: usize)->usize{
        (-self.parent[self.find(p)]) as usize
    }

    pub fn rollback(&mut self){
        if let Some((u, v, p1, p2)) = self.hist.pop(){
            if u < self.parent.len(){
                self.parent[u] = p1;
                self.parent[v] = p2;
            }
        }
    }
}
