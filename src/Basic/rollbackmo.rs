pub trait RollbackMoMonoid{
    type S: Clone;
    type T;
    type U;
    type V: Clone;
    type X: Clone+Default;
    fn init_t(n: usize, q: usize, a: &Vec<Self::S>, b: &Self::U)->Self::T;
    fn increase(t: &mut Self::T, s: &Self::S);
    fn snapshot(t: &mut Self::T);
    fn rollback(t: &mut Self::T);
    fn get(t: &Self::T, x: &Self::V)->Self::X;
}

pub fn solve_rollback_mo<M>(a: Vec<M::S>, x: &M::U, query: Vec<(usize, usize, M::V)>)->Vec<M::X> where M: RollbackMoMonoid{
    let (n, q) = (a.len(), query.len());
    let b = ((n as f64).sqrt() as usize).max(1);
    let mut ans = vec![M::X::default(); q];
    let mut qs = vec![Vec::new(); (n+b-1)/b];
    let mut t = M::init_t(n, q, &a, x);
    for (idx, (l, r, z)) in query.iter().enumerate(){
        let (bl, br) = ((*l+b-1)/b, *r/b);
        if bl >= br{
            for i in *l..*r{
                M::increase(&mut t, &a[i]);
            }
            ans[idx] = M::get(&t, z);
            M::rollback(&mut t);
        } else {
            qs[bl].push((*r, *l, z.clone(), idx));
        }
    }
    for i in 0..qs.len(){
        qs[i].sort_by(|u, v| u.0.cmp(&v.0));
        let st = (i+1)*b;
        let mut right = st;
        t = M::init_t(n, q, &a, x);
        for (r, l, z, idx) in &qs[i]{
            while right < *r{
                M::increase(&mut t, &a[right]);
                right += 1;
            }
            M::snapshot(&mut t);
            for i in (*l..st).rev(){
                M::increase(&mut t, &a[i]);
            }
            ans[*idx] = M::get(&mut t, z);
            M::rollback(&mut t);
        }
    }
    ans
}
