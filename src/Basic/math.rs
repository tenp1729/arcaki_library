#[inline(always)]
pub fn gcd(mut a: i64, mut b: i64)->i64{if a==0{return b;}else if b==0{return a;}let l1 = a.trailing_zeros();let l2 = b.trailing_zeros();
a >>= l1; b >>= l2;while a!=b{let x = (a^b).trailing_zeros();if a<b{swap(&mut a, &mut b)}a = (a-b)>>x;}a << l1.min(l2)}
pub fn factorial_i64(n: usize)->(Vec<i64>, Vec<i64>){ 
    let mut res = vec![1; n+1];let mut inv = vec![1; n+1];for i in 0..n{ res[i+1] = (res[i]*(i+1)as i64)%MOD; }
    inv[n] = mod_inverse(res[n], MOD);for i in (0..n).rev(){ inv[i] = inv[i+1]*(i+1) as i64%MOD; }(res, inv) }
pub fn floor(a:i64, b:i64)->i64{let res=(a%b+b)%b;(a-res)/b}
pub fn modulo(a: i64, b: i64)->i64{(a%b+b)%b}
pub fn extended_gcd(a:i64,b:i64)->(i64,i64,i64)
{if b==0{(a,1,0)}else{let(g,x,y)=extended_gcd(b,a%b);(g,y,x-floor(a,b)*y)}}
pub fn mod_inverse(a:i64,m:i64)->i64{let(_,x,_) =extended_gcd(a,m);(x%m+m)%m}
pub fn comb(a: i64, b: i64, f: &Vec<(i64, i64)>)->i64{
    if a<b{return 0;}else if b==0 || a==b{ return 1; }
    else{let x=f[a as usize].0;
        let y=f[(a-b) as usize].1;let z=f[b as usize].1;return((x*y)%MOD)*z%MOD;}}
pub fn factorial(x: i64)->Vec<(i64, i64)>{
    let mut f=vec![(1i64,1i64),(1, 1)];let mut z = 1i64;
    let mut inv = vec![0; x as usize+10];inv[1] = 1;
    for i in 2..x+1{z=(z*i)%MOD;
        let w=(MOD-inv[(MOD%i)as usize]*(MOD/i)%MOD)%MOD;
        inv[i as usize] = w;
        f.push((z, (f[i as usize-1].1*w)%MOD));}return f;}
pub fn fast_mod_pow(mut x: i64,p: usize, m: i64)->i64{
    x %= m;
    let mut res=1;let mut t=x;let mut z=p;while z > 0{
        if z%2==1{res = (res*t)%m;}t = (t*t)%m;z /= 2; }res}