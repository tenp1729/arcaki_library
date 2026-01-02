// 矩形[lx, rx (i64))[ly, ry)を問う質問クエリ(lx, ly, rx, ry)と点(lx, ly) += rx (i64)でry==!0の加算クエリに整形しておく
pub fn solve_point_add_rectangle_sum(width: usize, query: &mut Vec<(usize, usize, usize, usize, i64)>)->Vec<i64>{
    let mut bit = BIT::new(width, 0i64);
    let mut ans = vec![0; query.len()];
    let mut qs = Vec::with_capacity(query.len()*2);
    point_add_rectangle_sum_dfs(0, query.len(), &query, &mut bit, &mut qs, &mut ans);
    ans
}

#[inline]
fn point_add_rectangle_sum_dfs(l: usize, r: usize, query: &Vec<(usize, usize, usize, usize, i64)>, bit: &mut BIT<i64>, qs: &mut Vec<(usize, usize, usize, usize, i64)>, ans: &mut Vec<i64>){
    if l+1==r{return;}
    let m = (l+r)/2;
    point_add_rectangle_sum_dfs(l, m, query, bit, qs, ans);
    point_add_rectangle_sum_dfs(m, r, query, bit, qs, ans);
    for i in l..m{
        let (lx, ly, _, ry, w) = query[i];
        if ry==!0{
            qs.push((lx, !0, ly, 0, w));
        } 
    }
    for i in m..r{
        let (lx, ly, rx, ry, _) = query[i];
        if ry!=!0{
            qs.push((lx, i, ly, ry, 0));
            qs.push((rx, i+query.len(), ly, ry, 0));
        }
    }
    qs.sort_unstable();
    for &(_, mut idx, ly, ry, w) in qs.iter(){
        if idx == !0{
            bit.add(ly, w);
        } else {
            if idx < query.len(){
                ans[idx] -= bit.get(ly, ry);
            } else {
                idx -= query.len();
                ans[idx] += bit.get(ly, ry);
            }
        }
    }
    while let Some((_, idx, ly, _, w)) = qs.pop(){
        if idx == !0{
            bit.add(ly, w);
        } 
    }
}

// [l, r)の半開区間で設定
pub struct BIT<T> where T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
    n: usize,
    vec: Vec<T>,
    zero: T,
}

impl<T> BIT<T> where T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
    pub fn new(n: usize, zero: T) -> Self {
        let k = n.next_power_of_two();
        let base = vec![zero; k + 2];
        BIT { n: k, vec: base, zero }
    }

    #[inline]
    pub fn add(&mut self, mut idx: usize, x: T) {
        idx += 1;
        while idx <= self.n {
            self.vec[idx] = self.vec[idx] + x;
            idx += idx & (!idx + 1);
        }
    }

    #[inline]
    pub fn g(&mut self, mut r: usize) -> T {
        let mut res = self.zero;
        while r > 0 {
            res = res + self.vec[r];
            r -= r & (!r + 1);
        }
        res
    }

    #[inline]
    pub fn get(&mut self, l: usize, r: usize) -> T {
        self.g(r) - self.g(l)
    }
}
