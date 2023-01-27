use crate::{
    power_multiplicative_semigroup_with_std_ops::power,
    sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes,
};

pub fn sum_of_gcd(
    k: usize,
    n: u64,
) -> usize {
    let mut d = vec![0; k + 1];

    for i in 1..=k {
        d[i] = power(k / i, n);
    }

    for p in enumerate_primes(k as u32 + 1) {
        let p = p as usize;

        for i in (p..=k).step_by(p) {
            d[i / p] -= d[i];
        }
    }

    let mut s = 0;

    for i in 1..=k {
        s += i * d[i];
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [(2, 3, 9), (200, 3, 10813692)];

        for (k, n, ans) in cases {
            assert_eq!(sum_of_gcd(k, n), ans);
        }
    }
}
