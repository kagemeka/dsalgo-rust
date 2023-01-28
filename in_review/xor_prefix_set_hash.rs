use std::collections::{
    HashMap,
    HashSet,
};

use crate::rng_static_xorshift64::*;

/// pseudo random hash function: xorshift64
/// prefix set hashing method: cumulative xor

pub struct PrefixSetHash<T>(HashMap<T, u64>);

impl<T: Clone + std::hash::Hash + Eq> PrefixSetHash<T> {
    pub fn new() -> Self { Self(HashMap::new()) }

    pub fn calc(
        &mut self,
        a: &[T],
    ) -> Vec<u64> {
        let mut s = HashSet::new();

        let mut b: Vec<_> = a
            .iter()
            .map(|x| {
                let v = self
                    .0
                    .entry(x.clone())
                    .or_insert_with(|| static_xorshift64());

                if s.contains(x) {
                    0
                } else {
                    s.insert(x.clone());

                    *v
                }
            })
            .collect();

        for i in 0..a.len() - 1 {
            b[i + 1] ^= b[i];
        }

        b
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // https://atcoder.jp/contests/abc250/tasks/abc250_e
        let a = vec![1, 2, 3, 4, 5];

        let b = vec![1, 2, 2, 4, 3];

        let q = vec![
            ((1, 1), true),
            ((2, 2), true),
            ((2, 3), true),
            ((3, 3), false),
            ((4, 4), false),
            ((4, 5), true),
            ((5, 5), false),
        ];

        let mut hash = PrefixSetHash::new();

        let a = hash.calc(&a);

        let b = hash.calc(&b);

        for ((i, j), ans) in q {
            let i = i - 1;

            let j = j - 1;

            assert_eq!(a[i] == b[j], ans);
        }
    }
}
