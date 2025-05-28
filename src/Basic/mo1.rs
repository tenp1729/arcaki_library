// S: 配列の値
// T: 集計値
// U: クエリの値
// X: 出力値
// F: T*S→T
// G: T*U→X
pub trait MoMonoid{
    type S:Clone;
    type T;
    type U:Clone;
    type X:Clone+Default;
    fn id_t(n: usize, q: usize)->Self::T;
    fn increase(t: &mut Self::T, s: &Self::S);
    fn decrease(t: &mut Self::T, s: &Self::S);
    fn get(t: &Self::T, x: &Self::U)->Self::X;
}

pub fn solve_mo<M>(a: Vec<M::S>, query: Vec<(usize, usize, M::U)>)->Vec<M::X> where M: MoMonoid{
    let (n, q) = (a.len(), query.len());
    let div = (n as f64/(q as f64).sqrt()).ceil() as usize;
    let mut res = vec![M::X::default(); q];
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by(|&u, &v| (query[u].0/div).cmp(&(query[v].0/div)).then(if query[u].0/div%2==0{
        query[u].1.cmp(&query[v].1)
    } else {
        query[v].1.cmp(&query[u].1)
    }));
    let (mut l, mut r) = (0, 0);
    let mut b = M::id_t(n, q);
    for idx in ord{
        let (left, right) = (query[idx].0, query[idx].1);
        while r < right{
            M::increase(&mut b, &a[r]);
            r += 1;
        }
        while l > left{
            l -= 1;
            M::increase(&mut b, &a[l]);
        }
        while r > right{
            r -= 1;
            M::decrease(&mut b, &a[r]);
        }
        while l < left{
            M::decrease(&mut b, &a[l]);
            l += 1;
        }
        res[idx] = M::get(&b, &query[idx].2);
    }
    res
}

struct T{
    
}

struct M;
impl MoMonoid for M{
    type S = ();
    type T = ();
    type U = ();
    type X = ();

    fn id_t(n: usize, q: usize) -> Self::T {
        todo!()
    }

    fn increase(t: &mut Self::T, s: &Self::S) {
        todo!()
    }

    fn decrease(t: &mut Self::T, s: &Self::S) {
        todo!()
    }

    fn get(t: &Self::T, x: &Self::U) -> Self::X {
        todo!()
    }
}