pub struct CentroidDecomposition {
    n: usize,
    tree: Vec<Vec<usize>>,
    pre: Vec<usize>,
    level: Vec<usize>,
}

impl CentroidDecomposition {
    pub fn new(n: usize) -> Self {
        CentroidDecomposition {
            n,
            tree: vec![Vec::new(); n],
            pre: vec![!0; n],
            level: vec![!0; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.tree[u].push(v);
        self.tree[v].push(u);
    }

    pub fn build(&mut self) {
        use std::collections::VecDeque;
        let mut size = self.size_dfs();
        let mut stack = VecDeque::from([(0, !0, 0)]);
        for _ in 0..self.n {
            let (mut p, pre, d) = stack.pop_front().unwrap();
            let mut non = true;
            while non {
                non = false;
                for &nex in &self.tree[p] {
                    if self.level[nex] == !0 && size[nex] * 2 > size[p] {
                        size.swap(p, nex);
                        (size[p], p, non) = (size[nex] - size[p], nex, true);
                        break;
                    }
                }
            }
            self.pre[p] = pre;
            self.level[p] = d;
            if size[p] > 1 {
                for &nex in &self.tree[p] {
                    if self.level[nex] == !0 {
                        stack.push_back((nex, p, d+1));
                    }
                }
            }
        }
    }

    fn size_dfs(&mut self) -> Vec<usize> {
        use std::collections::VecDeque;
        let mut size = vec![1; self.n];
        let mut stack = VecDeque::from([(0, !0)]);
        let mut query = Vec::new();
        while let Some((p, pre)) = stack.pop_back() {
            for &nex in &self.tree[p] {
                if pre == nex {continue}
                stack.push_back((nex, p));
                query.push((nex, p));
            }
        }
        for &(p, pre) in query.iter().rev() {
            size[pre] += size[p];
        }
        size
    }

    pub fn lca(&mut self, mut u: usize, mut v: usize) -> usize {
        let (du, dv) = (self.level[u], self.level[v]);
        if du > dv {
            for _ in 0..du - dv {
                u = self.pre[u];
            }
        } else {
            for _ in 0..dv - du {
                v = self.pre[v];
            }
        }
        while u != v {
            (u, v) = (self.pre[u], self.pre[v])
        }
        u
    }

    pub fn roots(&mut self, mut v: usize) -> Vec<usize> {
        let mut res = Vec::from([v]);
        while self.pre[v] < self.n {
            v = self.pre[v];
            res.push(v);
        }
        res
    }

    pub fn root(&mut self, v: usize) -> usize {
        self.pre[v]
    }
}
