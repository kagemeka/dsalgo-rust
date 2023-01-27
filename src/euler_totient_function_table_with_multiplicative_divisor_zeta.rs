use crate::sieve_of_eratosthenes_enumerate_primes_usize::*;

/// O(N\log{N})

pub fn phi_table(size: usize) -> Vec<usize> {
    let mut u = vec![1; size];

    u[0] = 0;

    let mut d = vec![1; size];

    d[0] = 0;

    for p in enumerate_primes(size) {
        u[p] = p - 1;

        d[p] = p;
    }

    let zeta = |mut f: Vec<usize>| -> Vec<usize> {
        for i in (1..size).rev() {
            for j in (i << 1..size).step_by(i) {
                f[j] *= f[i];
            }
        }

        f
    };

    u = zeta(u);

    d = zeta(d);

    (0..size).map(|i| if i == 0 { 0 } else { i / d[i] * u[i] }).collect()
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
