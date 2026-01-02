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
    fn id_e() -> <Self::M as SegTreeMonoid>::S { <Self::M>::identity() }
    fn op(
        a: &<Self::M as SegTreeMonoid>::S,
        b: &<Self::M as SegTreeMonoid>::S
    ) -> <Self::M as SegTreeMonoid>::S { <Self::M>::op(a, b) }

    fn identity() -> Self::F;
    fn map(
        f: &Self::F,
        x: &<Self::M as SegTreeMonoid>::S
    ) -> <Self::M as SegTreeMonoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

#[derive(Clone)]
struct Node<F: LazySegtreeMonoid> {
    val: <F::M as SegTreeMonoid>::S,
    lazy: F::F,
    left: Option<usize>,
    right: Option<usize>,
}

pub struct DynamicLazySegtree<F: LazySegtreeMonoid> {
    size: usize,        
    nodes: Vec<Node<F>>, 
    root: usize,
}

impl<F: LazySegtreeMonoid> DynamicLazySegtree<F> {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let root = Node::<F> {
            val: F::id_e(),
            lazy: F::identity(),
            left: None,
            right: None,
        };
        Self { size, nodes: vec![root], root: 0 }
    }

    fn new_node(&mut self) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node::<F> {
            val: F::id_e(),
            lazy: F::identity(),
            left: None,
            right: None,
        });
        idx
    }

    fn ensure_left(&mut self, k: usize) -> usize {
        if let Some(c) = self.nodes[k].left { c }
        else {
            let c = self.new_node();
            self.nodes[k].left = Some(c);
            c
        }
    }

    fn ensure_right(&mut self, k: usize) -> usize {
        if let Some(c) = self.nodes[k].right { c }
        else {
            let c = self.new_node();
            self.nodes[k].right = Some(c);
            c
        }
    }

    fn apply_to_node(&mut self, k: usize, f: F::F) {
        let v = self.nodes[k].val.clone();
        self.nodes[k].val = F::map(&f, &v);
        self.nodes[k].lazy = F::composition(&f, &self.nodes[k].lazy);
    }

    fn push(&mut self, k: usize) {
        let f = self.nodes[k].lazy.clone();
        if f == F::identity() { return; }
        let lc = self.ensure_left(k);
        let rc = self.ensure_right(k);
        self.apply_to_node(lc, f.clone());
        self.apply_to_node(rc, f);
        self.nodes[k].lazy = F::identity();
    }

    fn pull(&mut self, k: usize) {
        let lv = if let Some(lc) = self.nodes[k].left {
            self.nodes[lc].val.clone()
        } else { F::id_e() };

        let rv = if let Some(rc) = self.nodes[k].right {
            self.nodes[rc].val.clone()
        } else { F::id_e() };

        self.nodes[k].val = F::op(&lv, &rv);
    }

    pub fn set(&mut self, p: usize, x: <F::M as SegTreeMonoid>::S) {
        assert!(p < self.size);
        self.set_rec(self.root, 0, self.size, p, x);
    }

    fn set_rec(
        &mut self,
        k: usize, l: usize, r: usize,
        p: usize, x: <F::M as SegTreeMonoid>::S
    ) {
        if r - l == 1 {
            self.nodes[k].val = x;
            self.nodes[k].lazy = F::identity();
            return;
        }
        self.push(k);
        let m = (l + r) >> 1;
        if p < m {
            let lc = self.ensure_left(k);
            self.set_rec(lc, l, m, p, x);
        } else {
            let rc = self.ensure_right(k);
            self.set_rec(rc, m, r, p, x);
        }
        self.pull(k);
    }

    pub fn get(&mut self, p: usize) -> <F::M as SegTreeMonoid>::S {
        assert!(p < self.size);
        self.get_rec(self.root, 0, self.size, p)
    }

    fn get_rec(
        &mut self,
        k: usize, l: usize, r: usize,
        p: usize
    ) -> <F::M as SegTreeMonoid>::S {
        if r - l == 1 {
            return self.nodes[k].val.clone();
        }
        self.push(k);
        let m = (l + r) >> 1;
        if p < m {
            if let Some(lc) = self.nodes[k].left {
                self.get_rec(lc, l, m, p)
            } else { F::id_e() }
        } else {
            if let Some(rc) = self.nodes[k].right {
                self.get_rec(rc, m, r, p)
            } else { F::id_e() }
        }
    }

    pub fn apply_range(&mut self, l: usize, r: usize, f: F::F) {
        if l >= r { return; }
        assert!(r <= self.size);
        self.apply_rec(self.root, 0, self.size, l, r, f);
    }

    fn apply_rec(
        &mut self,
        k: usize, nl: usize, nr: usize,
        ql: usize, qr: usize,
        f: F::F
    ) {
        if qr <= nl || nr <= ql { return; }
        if ql <= nl && nr <= qr {
            self.apply_to_node(k, f);
            return;
        }
        self.push(k);
        let m = (nl + nr) >> 1;
        let lc = self.ensure_left(k);
        let rc = self.ensure_right(k);
        self.apply_rec(lc, nl, m, ql, qr, f.clone());
        self.apply_rec(rc, m, nr, ql, qr, f);
        self.pull(k);
    }

    pub fn prod(&mut self, l: usize, r: usize) -> <F::M as SegTreeMonoid>::S {
        if l >= r { return F::id_e(); }
        assert!(r <= self.size);
        self.prod_rec(self.root, 0, self.size, l, r)
    }

    fn prod_rec(
        &mut self,
        k: usize, nl: usize, nr: usize,
        ql: usize, qr: usize
    ) -> <F::M as SegTreeMonoid>::S {
        if qr <= nl || nr <= ql { return F::id_e(); }
        if ql <= nl && nr <= qr { return self.nodes[k].val.clone(); }

        self.push(k);
        let m = (nl + nr) >> 1;

        let lv = if let Some(lc) = self.nodes[k].left {
            self.prod_rec(lc, nl, m, ql, qr)
        } else { F::id_e() };

        let rv = if let Some(rc) = self.nodes[k].right {
            self.prod_rec(rc, m, nr, ql, qr)
        } else { F::id_e() };

        F::op(&lv, &rv)
    }

    pub fn all_prod(&mut self) -> <F::M as SegTreeMonoid>::S {
        self.nodes[self.root].val.clone()
    }

    pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
    where
        G: Fn(<F::M as SegTreeMonoid>::S) -> bool
    {
        assert!(l <= self.size);
        assert!(g(F::id_e()));
        if l == self.size { return self.size; }
        let mut acc = F::id_e();
        self.max_right_rec(self.root, 0, self.size, l, &mut acc, &g)
    }

    fn max_right_rec<G>(
        &mut self,
        k: usize, nl: usize, nr: usize,
        l: usize,
        acc: &mut <F::M as SegTreeMonoid>::S,
        g: &G
    ) -> usize
    where
        G: Fn(<F::M as SegTreeMonoid>::S) -> bool
    {
        if nr <= l { return l; } 
        if l <= nl {
            let combined = F::op(acc, &self.nodes[k].val);
            if g(combined.clone()) {
                *acc = combined;
                return nr;
            }
            if nr - nl == 1 {
                return nl;
            }
        }
        self.push(k);
        let m = (nl + nr) >> 1;
        let lc = self.ensure_left(k);
        let res_l = self.max_right_rec(lc, nl, m, l, acc, g);
        if res_l < m { return res_l; }
        let rc = self.ensure_right(k);
        self.max_right_rec(rc, m, nr, l, acc, g)
    }

    pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
    where
        G: Fn(<F::M as SegTreeMonoid>::S) -> bool
    {
        assert!(r <= self.size);
        assert!(g(F::id_e()));
        if r == 0 { return 0; }
        let mut acc = F::id_e();
        self.min_left_rec(self.root, 0, self.size, r, &mut acc, &g)
    }

    fn min_left_rec<G>(
        &mut self,
        k: usize, nl: usize, nr: usize,
        r: usize,
        acc: &mut <F::M as SegTreeMonoid>::S,
        g: &G
    ) -> usize where G: Fn(<F::M as SegTreeMonoid>::S) -> bool {
        if r <= nl { return r; }

        if nr <= r {
            let combined = F::op(&self.nodes[k].val, acc);
            if g(combined.clone()) {
                *acc = combined;
                return nl;
            }
            if nr - nl == 1 {
                return nr;
            }
        }
        self.push(k);
        let m = (nl + nr) >> 1;
        let rc = self.ensure_right(k);
        let res_r = self.min_left_rec(rc, m, nr, r, acc, g);
        if res_r > m { return res_r; }
        let lc = self.ensure_left(k);
        self.min_left_rec(lc, nl, m, r, acc, g)
    }
}
