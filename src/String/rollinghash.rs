const B: u128 = 1001399;
const M: u128 = (1 << 64) - 59;

pub struct RollingHash {
    hash: Vec<u128>,
    pow: Vec<u128>,
}

impl RollingHash {
    pub fn char(s: &Vec<char>) -> Self {
        let (mut pow, mut hash) = (Vec::from([1]), Vec::from([0]));
        let (mut p, mut h) = (1, 0);
        for i in 0..s.len() {
            p = (p * B) % M;
            h = (h * B + s[i] as u128 - 'a' as u128 + 1)%M;
            pow.push(p);
            hash.push(h);
        }
        RollingHash {
            hash,
            pow,
        }
    }

    pub fn num_seq(a: &Vec<usize>) -> Self {
        let (mut pow, mut hash) = (Vec::from([1]), Vec::from([0]));
        let (mut p, mut h) = (1, 0);
        for i in 0..a.len() {
            p = (p * B) % M;
            h = (h * B + a[i] as u128 + 1) % M;
            pow.push(p);
            hash.push(h);
        }
        RollingHash {
            hash,
            pow,
        }
    }

    // mxは種類数上限。あと種類はちゃん全部列挙しないとなので配列が複数あるなら全部まとめてから座圧せよ。
    pub fn num_set(a: &Vec<usize>, mx: usize) -> Self {
        use std::collections::HashSet;
        let (mut pow, mut hash) = (Vec::from([1]), Vec::from([0]));
        let (mut p, mut h) = (1, 0);
        let mut used = HashSet::new();
        for _ in 0..mx {
            p = (p * B) % M;
            pow.push(p);
        }
        for i in 0..a.len() {
            if !used.contains(&a[i]) {
                used.insert(a[i]);
                h = (h + pow[a[i]]) % M;
            }
            hash.push(h);
        }
        RollingHash {
            hash,
            pow,
        }
    }

    pub fn num_map(a: &Vec<usize>, mx: usize) -> Self {
        let (mut pow, mut hash) = (Vec::from([1]), Vec::from([0]));
        let (mut p, mut h) = (1, 0);
        for _ in 0..mx {
            p = (p * B) % M;
            pow.push(p);
        }
        for i in 0..a.len() {
            h = (h + pow[a[i]]) % M;
            hash.push(h);
        }
        RollingHash {
            hash,
            pow,
        }
    }

    pub fn get(&mut self, l: usize, r: usize) -> u128 {
        (M + self.hash[r] - self.hash[l] * self.pow[r - l] % M) % M
    }

    pub fn map_get(&mut self, l: usize, r: usize) -> u128 {
        (M + self.hash[r] - self.hash[l] % M) % M
    }

    pub fn same(&mut self, l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        self.get(l1, r1) == self.get(l2, r2)
    }
}

pub fn compress_dic(a: &Vec<i64>) -> std::collections::HashMap<i64, usize> {
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut dic = std::collections::HashMap::new();
    for i in 0..b.len() {
        dic.insert(b[i], i);
    }
    dic
}
