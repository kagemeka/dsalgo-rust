use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    legendre_formula_recurse_usize::*,
    sieve_of_eratosthenes_enumerate_primes_usize::*,
};

/// n < 10^14, min(k, n - k) < 10^7

pub fn factorize_combination(
    n: usize,
    mut k: usize,
) -> Vec<(usize, usize)> {
    let mut factors = vec![];

    if k > n {
        return factors;
    }

    k = k.min(n - k);

    let lo = n - k + 1;

    let mut v: Vec<_> = (lo..=n).collect();

    for p in enumerate_primes(isqrt(n).max(k) + 1) {
        let e = legendre(n, p) - legendre(k, p) - legendre(n - k, p);

        if e > 0 {
            factors.push((p, e));
        }

        for i in ((lo + p - 1) / p * p..=n).step_by(p) {
            while v[i - lo] % p == 0 {
                v[i - lo] /= p;
            }
        }
    }

    for i in 0..k {
        if v[i] > 1 {
            factors.push((v[i], 1));
        }
    }

    factors.sort();

    factors
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((10, 5), vec![(2, 2), (3, 2), (7, 1)]),
            (
                (1_000_000_010, 5),
                vec![
                    (2, 2),
                    (3, 1),
                    (7, 1),
                    (17, 1),
                    (109, 2),
                    (167, 1),
                    (5882353, 1),
                    (500000003, 1),
                    (1000000007, 1),
                    (1000000009, 1),
                ],
            ),
        ];

        for ((n, k), ans) in cases {
            assert_eq!(factorize_combination(n, k), ans);
        }
    }
}
