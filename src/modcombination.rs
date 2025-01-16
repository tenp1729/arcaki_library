const MOD: i64 = 1_000_000_007;

pub fn floor(a:i64, b:i64)->i64{let res=(a%b+b)%b;(a-res)/b}
pub fn extended_gcd(a:i64,b:i64)->(i64,i64,i64)
{if b==0{(a,1,0)}else{let(g,x,y)=extended_gcd(b,a%b);(g,y,x-floor(a,b)*y)}}
pub fn mod_inverse(a:i64,m:i64)->i64{let(_,x,_) =extended_gcd(a,m);(x%m+m)%m}
pub fn comb(a: i64, b: i64, f: &Vec<(i64, i64)>)->i64{
    if a<b{return 0;}if b==0 || a==b{ return 1; }
    else{let x=f[a as usize].0;
        let y=f[(a-b) as usize].1;let z=f[b as usize].1;return((x*y)%MOD)*z%MOD;}}
pub fn factorial(x: i64)->Vec<(i64, i64)>{
    let mut f=vec![(1i64,1i64),(1, 1)];let mut z = 1i64;
    let mut inv = vec![0; x as usize+10];inv[1] = 1;
    for i in 2..x+1{z=(z*i)%MOD;
        let w=(MOD-inv[(MOD%i)as usize]*(MOD/i)%MOD)%MOD;
        inv[i as usize] = w;
        f.push((z, (f[i as usize-1].1*w)%MOD));}return f;}
pub fn fast_mod_pow(x: i64,p: usize, m: i64)->i64{
    let mut res=1;let mut t=x;let mut z=p;while z > 0{
        if z%2==1{res = (res*t)%m;}t = (t*t)%m;z /= 2; }res}
