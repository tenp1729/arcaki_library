pub fn closest_pair(ps: &mut Vec<(f64, f64)>) -> f64 {
    ps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.partial_cmp(&b.1).unwrap()));
    closest_find(ps)
}

fn closest_find(ps: &mut [(f64, f64)]) -> f64 {
    if ps.len() <= 1{
        return 1e60;
    }
    let m = ps.len() / 2;
    let center = ps[m].0;
    let mut res = closest_find(&mut ps[..m]);
    res = res.min(closest_find(&mut ps[m..]));
    let (mut idx1, mut idx2) = (0, 0);
    let mut nps = Vec::with_capacity(ps.len());
    while idx1 < m || idx2 + m < ps.len() {
        if idx1<m && (idx2 + m == ps.len() || ps[idx1].1 < ps[idx2 + m].1) {
            nps.push(ps[idx1]);
            idx1 += 1;
        } else {
            nps.push(ps[idx2 + m]);
            idx2 += 1;
        }
    }
    ps.copy_from_slice(&nps);
    let mut near: Vec<(f64, f64)> = Vec::new();
    for &(x, y) in ps.iter() {
        if (x - center).abs() > res {continue}
        for &(px, py) in near.iter().rev() {
            if res + py < y{break;}
            let (dx, dy) = ((x - px).abs(), y - py);
            res = res.min((dx * dx+ dy * dy).sqrt());
        }
        near.push((x, y));
    }
    res
}

