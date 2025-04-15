pub fn floor(a:i64, b:i64)->i64{
    let res=(a%b+b)%b;
    (a-res)/b
}

pub fn modulo(x: i64, y: i64)->i64{
    (x%y+y)%y
}

pub fn gcd(a: i64, b: i64)->i64{
    if b==0{
        a
    } else {
        gcd(b, modulo(a, b))
    }
}

pub fn ext_gcd(a: i64, b: i64)->(i64, i64, i64){
    if b==0{
        (a, 1, 0)
    } else {
        let (g, x, y) = ext_gcd(b, modulo(a, b));
        (g, y, x-floor(a, b)*y)
    }
}

pub fn crt(ss: &Vec<(usize, usize)>)->(usize, usize){
    let mut r = 0; let mut m = 1;
    for &(bi, mi) in ss{
        let (g, p, _) = ext_gcd(m, mi as i64);
        if (bi as i64-r)%g!=0{return (!0, !0);}
        let t = modulo(floor(bi as i64-r, g)*p, floor(mi as i64, g));
        r += m*t;
        m = floor(m*mi as i64, g);
        r = modulo(r, m);
    }
    (r as usize, m as usize)
}
