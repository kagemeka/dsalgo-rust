//! example
//! factorize(2, 8)
//! = [[(2, 1)], [(3, 1)], [(2, 2)], [(5, 1)], [(2, 1), (3, 1)], [(7, 1)]]

use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes,
};

pub struct RangeFactorize {
    primes: Vec<usize>,
}

impl RangeFactorize {
    pub fn new(limit: usize) -> Self {
        Self { primes: enumerate_primes(isqrt(limit) + 1) }
    }

    /// lo <= hi <= limit, hi - lo < 10^7

    pub fn factorize(
        &self,
        lo: usize,
        hi: usize,
    ) -> Vec<Vec<(usize, usize)>> {
        let mut v: Vec<_> = (lo..hi).collect();

        let n = hi - lo;

        let mut factors = vec![vec![]; n];

        for &p in self.primes.iter() {
            for i in ((lo + p - 1) / p * p..hi).step_by(p) {
                let mut e = 0;

                while v[i - lo] % p == 0 {
                    v[i - lo] /= p;

                    e += 1;
                }

                if e > 0 {
                    factors[i - lo].push((p, e));
                }
            }
        }

        for i in 0..n {
            if v[i] > 1 {
                factors[i].push((v[i], 1));
            }
        }

        factors
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (2, 8),
            vec![
                vec![(2, 1)],
                vec![(3, 1)],
                vec![(2, 2)],
                vec![(5, 1)],
                vec![(2, 1), (3, 1)],
                vec![(7, 1)],
            ],
        )];

        let f = RangeFactorize::new(100);

        for ((lo, hi), ans) in cases {
            assert_eq!(f.factorize(lo, hi), ans);
        }
    }
}
