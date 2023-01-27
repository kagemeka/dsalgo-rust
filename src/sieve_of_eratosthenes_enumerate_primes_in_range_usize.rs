//! example
//! enumerate(10, 20)
//! = [11, 13, 17, 19]

use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes,
};

/// lo <= hi <= 10^14, hi - lo < 10^7

pub fn range_primes(
    mut lo: usize,
    mut hi: usize,
) -> Vec<usize> {
    lo = lo.max(2);

    hi = hi.max(2);

    let n = hi - lo;

    let mut is_prime = vec![true; n];

    for p in enumerate_primes(isqrt(hi) + 1) {
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

    for ((lo, hi), ans) in cases {
        assert_eq!(range_primes(lo, hi), ans);
    }
}
