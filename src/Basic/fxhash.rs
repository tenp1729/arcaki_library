mod fxhash{
    use std::hash::BuildHasherDefault;

    #[derive(Default)]
    pub struct FxHasher{
        pub hash: u64,
    }
    impl std::hash::Hasher for FxHasher{
        #[inline(always)]
        fn finish(&self) -> u64 {
            self.hash
        }

        #[inline(always)]
        fn write(&mut self, bytes: &[u8]) {
            let mut h = self.hash;
            for &b in bytes{
                h = h.rotate_left(5)^(b as u64);
                h = h.wrapping_mul(0x517cc1b727220a95);
            }
            self.hash = h;
        }
    }
    pub type FxBuildHasher = BuildHasherDefault<FxHasher>;
    pub type FxMap<K, V> = std::collections::HashMap<K, V, FxBuildHasher>;
    pub type FxSet<K> = std::collections::HashSet<K, FxBuildHasher>;
}
//#[allow(unused_imports)]
//use fxhash::{FxSet, FxMap, FxBuildHasher};
