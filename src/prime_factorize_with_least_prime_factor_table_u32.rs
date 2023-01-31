use crate::least_prime_factor_table_with_sieve_of_eratosthenes_u32::*;

pub struct PrimeFactorize {
    lpf: Vec<Option<u32>>,
}

impl PrimeFactorize {
    pub fn new(size: usize) -> Self { Self { lpf: least_prime_factor(size) } }

    pub fn factorize(
        &self,
        mut n: u32,
    ) -> Vec<(u32, u32)> {
        assert!((n as usize) < self.lpf.len());

        let mut factors = vec![];

        while n > 1 {
            let p = self.lpf[n as usize].unwrap();

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

    #[test]

    fn test() {}
}
