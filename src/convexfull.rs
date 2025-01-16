pub fn cross_product(a: (i64, i64), b: (i64, i64), c: (i64, i64))->i64{
    (b.0-a.0)*(c.1-a.1)-(b.1-a.1)*(c.0-a.0)
}

pub fn convex_hull(point: &Vec<(i64, i64)>)-> (Vec<(i64, i64)>, Vec<(i64, i64)>){
    if point.len() < 3{
        let mut res = point.clone();
        res.sort();
        return (res.clone(), res.clone());
    }
    let mut sort_set = point.clone();
    sort_set.sort();
    let mut up_side = Vec::new();
    let mut low_side = Vec::new();
    for p in sort_set{
        while up_side.len() >= 2 &&
            cross_product(up_side[up_side.len()-2], up_side[up_side.len()-1], p) >= 0{
            up_side.pop();
        }
        up_side.push(p);
        while low_side.len() >= 2 &&
            cross_product(low_side[low_side.len()-2], low_side[low_side.len()-1], p) <= 0{
            low_side.pop();
        }
        low_side.push(p);
    }
    let res1 = up_side.iter().map(|&x| x).collect_vec();
    let res2 = low_side.iter().map(|&x| x).collect_vec();
    (res1, res2)
}
