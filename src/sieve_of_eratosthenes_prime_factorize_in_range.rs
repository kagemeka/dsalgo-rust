//! example
//! factorize(2, 8)
//! = [(2, 4), (3, 2), (5, 1), (7, 1)]

use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes,
};

/// lo <= hi < 10^14, hi - lo < 10^7

pub fn factorize(
    lo: usize,
    hi: usize,
) -> Vec<(usize, usize)> {
    let mut v: Vec<_> = (lo..hi).collect();

    let mut factors = vec![];

    for p in enumerate_primes(isqrt(hi) + 1) {
        let mut e = 0;

        for i in ((lo + p - 1) / p * p..hi).step_by(p) {
            while v[i - lo] % p == 0 {
                v[i - lo] /= p;

                e += 1;
            }
        }

        if e > 0 {
            factors.push((p, e));
        }
    }

    v.sort_unstable();

    let mut v = v.into_iter().skip_while(|&x| x == 1);

    if let Some(mut p) = v.next() {
        let mut e = 1;

        while let Some(x) = v.next() {
            if x == p {
                e += 1;
            } else {
                factors.push((p, e));

                p = x;

                e = 1;
            }
        }

        factors.push((p, e));
    }

    factors
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![((2, 8), vec![(2, 4), (3, 2), (5, 1), (7, 1)])];

        for ((lo, hi), ans) in cases {
            assert_eq!(factorize(lo, hi), ans);
        }
    }
}
