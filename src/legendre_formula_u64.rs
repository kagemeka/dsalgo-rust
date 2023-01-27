/// number of a prime factor in factorial.

pub fn legendre(
    mut n: u64,
    p: u64,
) -> u64 {
    let mut v = 0;

    while n > 0 {
        v += n / p;

        n /= p;
    }

    v
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
