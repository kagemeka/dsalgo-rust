use crate::sieve_of_eratosthenes_least_prime_factor_table_usize_optim2::*;

pub struct PrimeFactorize(Vec<usize>);

impl PrimeFactorize {
    pub fn new(size: usize) -> Self { Self(least_prime_factor(size)) }

    pub fn factorize(
        &self,
        mut n: usize,
    ) -> Vec<(usize, usize)> {
        assert!(n < self.0.len());

        let mut factors = vec![];

        while n > 1 {
            let p = self.0[n];

            let mut e = 0;

            while n % p == 0 {
                n /= p;

                e += 1;
            }

            factors.push((p, e));
        }

        factors
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::prime_factorize_trial_division_usize::prime_factorize;

        let f = PrimeFactorize::new(1 << 10);

        for i in 1..1 << 10 {
            assert_eq!(f.factorize(i), prime_factorize(i));
        }
    }
}
