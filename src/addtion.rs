pub fn get_sqrt(x: i64) -> i64 {
    let mut res = (x as f64).sqrt() as i64 - 1;
    while res < 0 || (res + 1) * (res + 1) <= x{
        res += 1;
    }
    res
}
