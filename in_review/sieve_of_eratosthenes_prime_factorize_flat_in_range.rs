use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes,
};

/// lo <= hi < 10^14, hi - lo < 10^7

pub fn factorize(
    lo: usize,
    hi: usize,
) -> Vec<usize> {
    let mut v: Vec<_> = (lo..hi).collect();

    let mut factors = Vec::with_capacity((hi - lo) << 2);

    for p in enumerate_primes(isqrt(hi) + 1) {
        for i in ((lo + p - 1) / p * p..hi).step_by(p) {
            while v[i - lo] % p == 0 {
                v[i - lo] /= p;

                factors.push(p);
            }
        }
    }

    v.sort_unstable();

    factors.extend(v.into_iter().skip_while(|x| x == &1));

    factors
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![((2, 8), vec![2, 2, 2, 2, 3, 3, 5, 7])];

        for ((lo, hi), ans) in cases {
            assert_eq!(factorize(lo, hi), ans);
        }
    }
}
