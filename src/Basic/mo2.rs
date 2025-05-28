pub trait MoMonoid{
    type S1:Clone;
    type S2:Clone;
    type T;
    type U:Clone;
    type X:Clone;
    fn id_t(n1: usize, n2: usize, q: usize)->Self::T;
    fn id_x(n1: usize, n2: usize, q: usize)->Self::X;
    fn l_increase(t: &mut Self::T, s: &Self::S1);
    fn r_increase(t: &mut Self::T, s: &Self::S2);
    fn l_decrease(t: &mut Self::T, s: &Self::S1);
    fn r_decrease(t: &mut Self::T, s: &Self::S2);
    fn get(t: &Self::T, x: &Self::U)->Self::X;
}

pub fn solve_mo_2d<M>(a1: Vec<M::S1>, a2: Vec<M::S2>, query: Vec<(usize, usize, M::U)>)->Vec<M::X> where M: MoMonoid{
    let n = a1.len()+a2.len();
    let q = query.len();
    let mut res = vec![M::id_x(a1.len(), a2.len(), query.len()); q];
    let mut ord = (0..query.len()).collect::<Vec<_>>();
    let div = (n as f64/(q as f64).sqrt()).ceil() as usize;
    ord.sort_by(|&u, &v| (query[u].0/div).cmp(&(query[v].0/div)).then(if query[u].0/div%2==0{
        query[u].1.cmp(&query[v].1)
    } else {
        query[v].1.cmp(&query[u].1)
    }));
    let (mut l, mut r) = (0, 0);
    let mut b = M::id_t(a1.len(), a2.len(), query.len());
    for idx in ord{
        let (left, right) = (query[idx].0, query[idx].1);
        while r < right{
            M::r_increase(&mut b, &a2[r]);
            r += 1;
        }
        while l > left{
            l -= 1;
            M::l_decrease(&mut b, &a1[l]);
        }
        while r > right{
            r -= 1;
            M::r_decrease(&mut b, &a2[r]);
        }
        while l < left{
            M::l_increase(&mut b, &a1[l]);
            l += 1;
        }
        res[idx] = M::get(&b, &query[idx].2);
    }
    res
}

struct T {
    
}

struct M;
impl MoMonoid for M{
    type S1 = ();
    type S2 = ();
    type T = T;
    type U = ();
    type X = ();

    fn id_t(n1: usize, n2: usize, q: usize) -> Self::T {
        todo!()
    }

    fn id_x(n1: usize, n2: usize, q: usize) -> Self::X {
        todo!()
    }

    fn l_increase(t: &mut Self::T, s: &Self::S1) {
        todo!()
    }

    fn r_increase(t: &mut Self::T, s: &Self::S2) {
        todo!()
    }

    fn l_decrease(t: &mut Self::T, s: &Self::S1) {
        todo!()
    }

    fn r_decrease(t: &mut Self::T, s: &Self::S2) {
        todo!()
    }

    fn get(t: &Self::T, x: &Self::U) -> Self::X {
        todo!()
    }
}
