use crate::sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes;

/// O(N\log{\log{N}})

pub fn sum_of_multiples(size: usize) -> Vec<usize> {
    let mut f: Vec<_> = (0..size).collect();

    for p in enumerate_primes(size) {
        for i in (1..(size - 1) / p + 1).rev() {
            f[i] += f[i * p];
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let f = sum_of_multiples(10);

        assert_eq!(f, [0, 45, 20, 18, 12, 5, 6, 7, 8, 9]);
    }
}
