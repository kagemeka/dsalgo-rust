//! example
//! enumerate(10, 20)
//! = [11, 13, 17, 19]

use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes,
};

pub struct EnumerateRangePrimes(Vec<usize>);

impl EnumerateRangePrimes {
    /// limit < 10^14

    pub fn new(limit: usize) -> Self {
        Self(enumerate_primes(isqrt(limit) + 1))
    }

    /// lo <= hi <= limit, hi - lo < 10^7

    pub fn enumerate(
        &self,
        mut lo: usize,
        mut hi: usize,
    ) -> Vec<usize> {
        lo = lo.max(2);

        hi = hi.max(2);

        let n = hi - lo;

        let mut is_prime = vec![true; n];

        for &p in self.0.iter() {
            for i in ((p * p).max((lo + p - 1) / p * p)..hi).step_by(p) {
                is_prime[i - lo] = false;
            }
        }

        is_prime
            .into_iter()
            .enumerate()
            .filter_map(|(i, ok)| if ok { Some(i + lo) } else { None })
            .collect()
    }
}

#[test]

fn test() {
    let cases = vec![
        (
            (100, 500),
            vec![
                101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227,
                229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283,
                293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367,
                373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439,
                443, 449, 457, 461, 463, 467, 479, 487, 491, 499,
            ],
        ),
        ((0, 10), vec![2, 3, 5, 7]),
    ];

    let f = EnumerateRangePrimes::new(1 << 10);

    for ((lo, hi), ans) in cases {
        assert_eq!(f.enumerate(lo, hi), ans);
    }
}
