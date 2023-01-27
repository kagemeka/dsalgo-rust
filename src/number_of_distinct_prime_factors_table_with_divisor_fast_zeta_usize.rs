use crate::sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes;

pub fn number_of_prime_factors(size: usize) -> Vec<usize> {
    let mut f = vec![0; size];

    let primes = enumerate_primes(size);

    for &p in enumerate_primes(size).iter() {
        f[p] = 1;
    }

    for p in primes {
        for i in 1..(size - 1) / p + 1 {
            f[i * p] += f[i];
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const ANS: &[usize] = &[
            0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2,
            1, 2, 1, 2, 1, 2, 1, 3, 1, 1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 3, 1, 2,
            2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 3,
            1, 2, 2, 3, 1, 2, 1, 2, 2, 2, 2, 3, 1, 2, 1, 2, 1, 3, 2, 2, 2, 2,
            1, 3, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 1, 3, 1, 2, 3, 2, 1, 2, 1, 3,
            2,
        ];

        let n = ANS.len();

        assert_eq!(&number_of_prime_factors(n + 1)[1..], ANS);
    }
}
