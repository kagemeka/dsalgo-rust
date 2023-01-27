use crate::sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes;

/// use \sum_{d|n} mu(n) = 1 if n = 1 else 0

pub fn mobius_function(size: usize) -> Vec<isize> {
    let mut f = vec![0; size];

    f[1] = 1;

    for p in enumerate_primes(size) {
        for i in (1..(size - 1) / p + 1).rev() {
            f[i * p] -= f[i];
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MU: &[isize] = &[
            1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0,
            1, 1, -1, 0, 0, 1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0,
            -1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1, 0, -1, 0, 1, 0, 1, 1, -1, 0,
            -1, 1, 0, 0, 1, -1, -1, 0, 1, -1, -1, 0, -1, 1, 0, 0, 1, -1,
        ];

        let n = MU.len();

        let mu = mobius_function(n + 1);

        assert_eq!(&mu[1..], MU);
    }
}
