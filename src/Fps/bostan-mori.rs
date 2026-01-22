pub mod fps {
    use super::{convolution, Mint};
    #[inline]
    pub fn zero() -> Mint { Mint::new(0) }
    #[inline]
    pub fn one() -> Mint { Mint::new(1) }
    #[inline]
    pub fn two() -> Mint { Mint::new(2) }

    #[inline]
    pub fn trim(mut a: Vec<Mint>) -> Vec<Mint> {
        while a.last().map_or(false, |x| *x == zero()) {
            a.pop();
        }
        a
    }

    #[inline]
    fn prefix(mut a: Vec<Mint>, n: usize) -> Vec<Mint> {
        if a.len() > n { a.truncate(n); }
        else { a.resize(n, zero()); }
        a
    }

    pub fn add(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
        let n = a.len().max(b.len());
        let mut c = vec![zero(); n];
        for i in 0..n {
            if i < a.len() { c[i] += a[i]; }
            if i < b.len() { c[i] += b[i]; }
        }
        trim(c)
    }

    pub fn sub(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
        let n = a.len().max(b.len());
        let mut c = vec![zero(); n];
        for i in 0..n {
            if i < a.len() { c[i] += a[i]; }
            if i < b.len() { c[i] -= b[i]; }
        }
        trim(c)
    }

    pub fn mul(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
        if a.is_empty() || b.is_empty() { return vec![]; }
        convolution(a, b)
    }

    /// Formal power series inverse: g(x) = 1/f(x) mod x^n
    /// Precondition: f[0] != 0
    pub fn inv(f: &[Mint], n: usize) -> Vec<Mint> {
        assert!(n > 0);
        assert!(!f.is_empty() && f[0] != zero());

        let mut g = vec![f[0].inv()]; // length 1
        let mut m = 1usize;

        while m < n {
            let _m2 = (m << 1).min(n.next_power_of_two()); // just a safe upper; we truncate anyway
            let need = (m << 1).min(n); // we only need up to 2m terms (capped by n)

            let f_tr = prefix(f.to_vec(), need);
            let mut t = mul(&f_tr, &g);
            t = prefix(t, need); // t = f * g mod x^need

            // t = 2 - t
            for i in 0..need { t[i] = -t[i]; }
            t[0] += two();

            g = mul(&g, &t);
            g = prefix(g, need);

            m <<= 1;
        }

        prefix(g, n)
    }

    /// Compute remainder p mod q (polynomial), using NTT + inv on reversed polynomial.
    /// Requires q != 0.
    pub fn poly_mod(p: &[Mint], q: &[Mint]) -> Vec<Mint> {
        let mut p = trim(p.to_vec());
        let q = trim(q.to_vec());
        assert!(!q.is_empty());

        if p.len() < q.len() {
            return p;
        }

        // degree
        let n = p.len() - 1;
        let m = q.len() - 1;
        let k = n - m + 1; // quotient length

        // reverse
        let mut rp = p.clone(); rp.reverse();
        let mut rq = q.clone(); rq.reverse();

        // inv(rq) up to k
        assert!(rq[0] != zero());
        let inv_rq = inv(&rq, k);

        // q_rev = (rp[0..k] * inv_rq)[0..k]
        let rp_k = prefix(rp, k);
        let mut qrev = mul(&rp_k, &inv_rq);
        qrev = prefix(qrev, k);

        // quotient = reverse(qrev)
        qrev.reverse();
        let quo = qrev;

        // r = p - quo*q
        let mut prod = mul(&quo, &q);
        prod.resize(p.len(), zero());

        for i in 0..p.len() {
            p[i] -= prod[i];
        }
        p.truncate(m);
        trim(p)
    }

    #[inline]
    fn even_coeffs(a: &[Mint]) -> Vec<Mint> {
        let mut res = Vec::with_capacity((a.len() + 1) / 2);
        for i in (0..a.len()).step_by(2) { res.push(a[i]); }
        trim(res)
    }

    #[inline]
    fn odd_coeffs(a: &[Mint]) -> Vec<Mint> {
        let mut res = Vec::with_capacity(a.len() / 2);
        for i in (1..a.len()).step_by(2) { res.push(a[i]); }
        trim(res)
    }

    #[inline]
    fn negate_odd(mut q: Vec<Mint>) -> Vec<Mint> {
        // q(-x): odd index negated
        for i in (1..q.len()).step_by(2) { q[i] = -q[i]; }
        q
    }

    /// Bostan–Mori:
    /// returns [x^k] P(x)/Q(x).
    ///
    /// Requirements (standard):
    /// - Q[0] != 0
    /// - Typically deg P < deg Q (if not, reduce by poly_mod first)
    pub fn bostan_mori(mut p: Vec<Mint>, mut q: Vec<Mint>, mut k: u64) -> Mint {
        p = trim(p);
        q = trim(q);
        assert!(!q.is_empty() && q[0] != zero());

        // Normalize so q[0] = 1 (optional but nice)
        let inv_q0 = q[0].inv();
        for x in p.iter_mut() { *x *= inv_q0; }
        for x in q.iter_mut() { *x *= inv_q0; }

        // If deg P >= deg Q, reduce.
        if p.len() >= q.len() {
            p = poly_mod(&p, &q);
        }

        while k > 0 {
            let q_neg = negate_odd(q.clone()); // Q(-x)

            let u = mul(&p, &q_neg); // P(x)Q(-x)
            let v = mul(&q, &q_neg); // Q(x)Q(-x) : only even degrees survive

            if (k & 1) == 0 {
                p = even_coeffs(&u);
            } else {
                p = odd_coeffs(&u);
            }
            q = even_coeffs(&v);

            // renormalize q[0]=1 to keep stable
            assert!(!q.is_empty() && q[0] != zero());
            let inv_q0 = q[0].inv();
            for x in p.iter_mut() { *x *= inv_q0; }
            for x in q.iter_mut() { *x *= inv_q0; }

            k >>= 1;
            if p.is_empty() { return zero(); }
        }

        // k==0 => coefficient is P(0)/Q(0) but we normalized Q(0)=1.
        if p.is_empty() { zero() } else { p[0] }
    }
}
use fps::bostan_mori;
