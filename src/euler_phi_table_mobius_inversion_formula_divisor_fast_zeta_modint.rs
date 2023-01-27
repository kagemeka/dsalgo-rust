use crate::{
    mobius_function_table_with_divisor_fast_mobius_isize::*,
    modular_inverse_table_usize::*,
    sieve_of_eratosthenes_enumerate_primes_usize::*,
};

/// O(N\log{\log{N}})
/// use \phi(n) = n\sum_{d|n}{mu(d)/d}
/// size must be <= 1_000_000_007

pub fn phi_table(size: usize) -> Vec<usize> {
    const MOD: isize = 1_000_000_007; // arbitrary enough big prime.
    let inv = inverse(MOD as usize, size);

    let mu = mobius_function(size);

    let mut f: Vec<_> = (0..size).map(|d| mu[d] * inv[d] as isize).collect();

    for p in enumerate_primes(size) {
        for i in 1..(size - 1) / p + 1 {
            f[i * p] += f[i];

            f[i * p] %= MOD;
        }
    }

    (0..size)
        .map(|i| (((i as isize * f[i] % MOD) + MOD) % MOD) as usize)
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const PHI: &[usize] = &[
            0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8,
            12, 10, 22, 8, 20, 12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36,
            18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20, 32, 24, 52, 18,
            40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
        ];

        let n = PHI.len();

        assert_eq!(phi_table(n), PHI);
    }
}
