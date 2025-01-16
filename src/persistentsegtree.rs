pub fn bit_length(x: usize)->usize{
    64-x.saturating_sub(1).leading_zeros()as usize
}

// 別にCopyじゃなくてCloneでもいいけどあんまり使わないと思われる…
pub trait SegTreeMonoid{
    type S: Copy;
    fn identity()->Self::S;
    fn op(a: &Self::S, b: &Self::S)->Self::S;
}

// 計算に交換法則を要求しているので注意。
// 初期化時にノードを作らずNoneなら単位元を返す実装なら
// 動的セグ木との兼用がおそらく可能でその場合10^9サイズとかでも問題ない。
pub struct PersistentSegmenttree<M> where M: SegTreeMonoid{
    n: usize, log: usize,
    data: Vec<(M::S, Option<usize>, Option<usize>)>,
    root: Vec<usize>,
}

impl<M> PersistentSegmenttree<M> where M: SegTreeMonoid{
    pub fn new(n: usize)->Self{
        let k = n.next_power_of_two();
        let log = bit_length(k);
        let mut vec = Vec::with_capacity(2*k);
        vec.push((M::identity(), None, None));
        for i in 1..k{
            vec.push((M::identity(), Some(2*i), Some(2*i+1)));
        }
        for _ in 0..k{
            vec.push((M::identity(), None, None));
        }
        let root = vec![1];
        PersistentSegmenttree{
            n: k, log, data: vec, root,
        }
    }

    pub fn build(vec: &Vec<M::S>)->Self{
        let n = vec.len().next_power_of_two();
        let log = bit_length(n);
        let mut data = vec![(M::identity(), None, None); 2*n];
        for (i, &v) in vec.iter().enumerate(){
            data[n+i] = (v, None, None);
        }
        for i in (1..n).rev(){
            data[i] = (M::op(&data[2*i].0, &data[2*i+1].0), Some(2*i), Some(2*i+1));
        }
        let root = vec![1];
        PersistentSegmenttree{
            n, log, data, root,
        }
    }

    pub fn update(&mut self, t: usize, mut p: usize, x: M::S){
        let nex = self.data.len();
        self.root.push(nex);
        let mut pp = self.root[t];
        p += self.n;
        for i in 0..self.log{
            let (_, l, r) = self.data[pp];
            if p & 1<<(self.log-i-1) == 0{
                self.data.push((M::identity(), Some(nex+i+1), r));
                pp = l.unwrap();
            } else {
                self.data.push((M::identity(), l, Some(nex+i+1)));
                pp = r.unwrap();
            }
        }
        self.data.push((x, None, None));
        for i in (0..self.log).rev(){
            let (_, l, r) = self.data[nex+i];
            let (left, right) = (l.unwrap(), r.unwrap());
            let res = M::op(&self.data[left].0, &self.data[right].0);
            self.data[nex+i] = (res, l, r);
        }
    }

    pub fn prod(&self, t: usize, l: usize, r: usize)->M::S{
        let mut res = M::identity();
        let p = self.root[t];
        self.dfs1(p, l, r, 0, self.n, &mut res);
        res
    }

    fn dfs1(&self, p: usize, l: usize, r: usize, x: usize, y: usize, res: &mut M::S){
        let (z, left, right) = self.data[p];
        if l <= x && y <= r{
            *res = M::op(res, &z);
            return;
        }
        let mid = (x+y)/2;
        if mid > l{
            self.dfs1(left.unwrap(), l, r, x, mid, res);
        }
        if mid < r{
            self.dfs1(right.unwrap(), l, r, mid, y, res);
        }
    }

    pub fn max_right<F>(&self, t: usize, l: usize, f: F)->usize where F: Fn(&M::S)->bool{
        assert!(f(&M::identity()));
        if l==self.n{return self.n}
        let mut ac = M::identity();
        self.dfs2(self.root[t], 0, self.n, l, &mut ac, &f)
    }

    fn dfs2<F>(&self, p: usize, l: usize, r: usize, x: usize, ac: &mut M::S, f: &F)->usize where F: Fn(&M::S)->bool{
        if r <= x{
            return x
        }
        if l >= x{
            let res = M::op(ac, &self.data[p].0);
            if f(&res){
                *ac = res;
                return r;
            } else if r-l==1{
                return l;
            }
        }
        let m = (l+r)/2;
        let (_, left, right) = self.data[p];
        let ret = self.dfs2(left.unwrap(), l, m, x, ac, f);
        if ret < m{
            return ret;
        }
        self.dfs2(right.unwrap(), m, r, x, ac, f)
    }

    // 動作がバグってる恐れあり
    pub fn min_left<F>(&self, t: usize, r: usize, f: F)->usize where F: Fn(&M::S)->bool{
        assert!(f(&M::identity()));
        let p = self.root[t];
        if r==0{return 0;}
        let mut ac = M::identity();
        self.dfs3(p, 0, self.n, r, &mut ac, &f)
    }

    fn dfs3<F>(&self, p: usize, l: usize, r: usize, x: usize, ac: &mut M::S, f: &F)->usize where F: Fn(&M::S)->bool{
        if x <= l{
            return l;
        } else if r <= x{
            let res = M::op(&self.data[p].0, ac);
            if f(&res){
                *ac = res;
                return l;
            } else if l+1==r{
                return r;
            }
        }
        let m = (l+r)/2;
        let (_, left, right) = self.data[p];
        let ret = self.dfs3(right.unwrap(), m, r, x, ac, f);
        if ret > m{
            ret
        } else {
            self.dfs3(left.unwrap(), l, m, x, ac, f)
        }
    }
}
