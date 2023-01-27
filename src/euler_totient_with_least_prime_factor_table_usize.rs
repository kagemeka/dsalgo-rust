use crate::sieve_of_eratosthenes_least_prime_factor_table_usize::*;

pub struct EulerTotient(Vec<usize>);

impl EulerTotient {
    pub fn new(less_than: usize) -> Self { Self(least_prime_factor(less_than)) }

    pub fn phi(
        &self,
        mut n: usize,
    ) -> usize {
        let mut phi = n;

        while n > 1 {
            let p = self.0[n];

            phi -= phi / p;

            while n % p == 0 {
                n /= p;
            }
        }

        phi
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let ans = [
            1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12,
            10, 22, 8, 20, 12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18,
            24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20, 32, 24, 52, 18, 40,
            24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
        ];

        let f = EulerTotient::new(100);

        for i in 0..ans.len() {
            assert_eq!(f.phi(i + 1), ans[i]);
        }
    }
}
