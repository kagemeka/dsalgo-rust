/// number of a prime factor in factorial.

pub fn legendre(
    n: u64,
    p: u64,
) -> u64 {
    if n == 0 {
        0
    } else {
        n / p + legendre(n / p, p)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::sieve_of_eratosthenes_prime_factorize_factorial::*;

        let n = 100_000;

        let factors = factorize_factorial(n);

        for (p, e) in factors {
            assert_eq!(legendre(n as u64, p as u64), e as u64);
        }
    }
}
