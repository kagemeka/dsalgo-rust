use crate::sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes;

/// O(N\log{\log{N}})

pub fn number_of_multiples(size: usize) -> Vec<usize> {
    let mut f = vec![1; size];

    f[0] = 0;

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
        let f = number_of_multiples(10);

        assert_eq!(f, [0, 9, 4, 3, 2, 1, 1, 1, 1, 1]);
    }
}
