pub fn bit_length(x: usize) -> usize {
    64 - x.saturating_sub(1).leading_zeros() as usize
}

pub trait SegTreeMonoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub trait LazySegtreeMonoid {
    type M: SegTreeMonoid;
    type F: Clone;
    fn id_e() -> <Self::M as SegTreeMonoid>::S { <Self::M as SegTreeMonoid>::identity() }
    fn op(a: &<Self::M as SegTreeMonoid>::S, b: &<Self::M as SegTreeMonoid>::S) -> <Self::M as SegTreeMonoid>::S { <Self::M>::op(a, b) }
    fn identity() -> Self::F;
    fn map(f: &Self::F, x: &<Self::M as SegTreeMonoid>::S) -> <Self::M as SegTreeMonoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

pub struct LazySegtree<F> where F: LazySegtreeMonoid {
    n: usize,
    log: usize,
    data: Vec<<F::M as SegTreeMonoid>::S>,
    lazy: Vec<F::F>,
}

impl<F: LazySegtreeMonoid> LazySegtree<F> {
    // 初期値開始
    pub fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        let log = bit_length(n);
        let lazy = vec![F::identity(); n << 1];
        let data = vec![F::id_e(); n << 1];
        LazySegtree {
            n, log, data, lazy,
        }
    }

    // vectorを飲ませるならこっち。O(N)で初期化。
    pub fn build(vec: &Vec<<F::M as SegTreeMonoid>::S>) -> Self {
        let n = vec.len().next_power_of_two();
        let log = bit_length(n);
        let lazy = vec![F::identity(); n<<1];
        let mut data = vec![F::id_e(); n << 1];
        data[n..(n + vec.len())].clone_from_slice(vec);
        let mut res = LazySegtree {
            n, log, data, lazy,
        };
        for i in (1..n).rev() {
            res.update(i);
        }
        res
    }

    pub fn set(&mut self, mut p: usize, x: <F::M as SegTreeMonoid>::S) {
        p += self.n;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    // 下からデータ更新
    fn update(&mut self, k: usize) {
        self.data[k] = F::op(&self.data[2 * k], &self.data[2 * k + 1]);
    }

    // 遅延反映
    fn inner_apply(&mut self, k: usize, f: F::F) {
        self.data[k] = F::map(&f, &self.data[k]);
        if k < self.n { self.lazy[k] = F::composition(&f, &self.lazy[k]) }
    }

    // 上から遅延更新
    fn push(&mut self, k: usize) {
        self.inner_apply(2 * k, self.lazy[k].clone());
        self.inner_apply(2 * k + 1, self.lazy[k].clone());
        self.lazy[k] = F::identity();
    }

    pub fn get(&mut self, mut p: usize) -> <F::M as SegTreeMonoid>::S {
        p += self.n;
        for i in (1..self.log).rev() {
            self.push(p >> i);
        }
        self.data[p].clone()
    }

    // whileで打ち切った方が早そうだけどどうなんでしょう？
    pub fn prod(&mut self, mut l: usize, mut r: usize) -> <F::M as SegTreeMonoid>::S {
        if r <= l { return F::id_e() }
        l += self.n; r += self.n;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }
        let mut acl = F::id_e();
        let mut acr = F::id_e();
        while l < r{
            if l & 1 != 0 {
                acl = F::op(&acl, &self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                acr = F::op(&self.data[r], &acr);
            }
            l >>= 1; r >>= 1;
        }
        F::op(&acl, &acr)
    }

    pub fn all_prod(&mut self) -> <F::M as SegTreeMonoid>::S {
        self.update(1);
        self.data[1].clone()
    }

    pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: F::F) {
        if l >= r { return; }
        l += self.n; r += self.n;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let left = l;
        let right = r;
        while l < r {
            if l & 1 != 0 {
                self.inner_apply(l, f.clone());
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.inner_apply(r, f.clone());
            }
            l >>= 1; r >>= 1;
        }
        for i in 1..=self.log {
            if ((left >> i) << i)!=left {
                self.update(left >> i);
            }
            if ((right >> i) << i) != right {
                self.update((right - 1) >> i);
            }
        }
    }

    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where G: Fn(<F::M as SegTreeMonoid>::S) -> bool {
        assert!(g(F::id_e()));
        if l >= self.n { return self.n }
        l += self.n;
        for i in 1..=self.log {
            self.push(l >> i);
        }
        let mut ac = F::id_e();
        while {
            while l % 2 == 0{
                l >>= 1;
            }
            if !g(F::op(&ac, &self.data[l])) {
                while l < self.n {
                    self.push(l);
                    l *= 2;
                    let res = F::op(&ac, &self.data[l]);
                    if g(res.clone()) {
                        ac = res;
                        l += 1;
                    }
                }
                return l - self.n;
            }
            ac = F::op(&ac, &self.data[l]);
            l += 1;
            let left = l as isize;
            (left & -left) != left
        } {}
        self.n
    }

    pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
    where G: Fn(<F::M as SegTreeMonoid>::S) -> bool {
        assert!(g(F::id_e()));
        if r == 0 { return 0; }
        r += self.n;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut ac = F::id_e();
        while {
            r -= 1;
            while r % 2 != 0 {
                r >>= 1;
            }
            if !g(F::op(&self.data[r], &ac)) {
                while r < self.n{
                    self.push(r);
                    r = 2 * r + 1;
                    let res = F::op(&self.data[r], &ac);
                    if g(res.clone()) {
                        ac = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }
            ac = F::op(&self.data[r], &ac);
            let right = r as isize;
            (right & -right) != right
        } {}
        0
    }

    pub fn get_slice(&mut self, mut l: usize, mut r: usize) -> Vec<<F::M as SegTreeMonoid>::S> {
        l += self.n; r += self.n;
        for i in 1..self.n {
            self.push(i)
        }
        (l..r).into_iter().map(|z| self.data[z].clone()).collect()
    }
}
