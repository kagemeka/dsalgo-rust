use crate::{
    modular_factorial_table_usize::*,
    modular_inverse_factorial_table_usize::*,
};

pub struct FactorialTablesFrequentOps {
    pub p: usize,
    pub fact: Vec<usize>,
    pub inv_fact: Vec<usize>,
}

impl FactorialTablesFrequentOps {
    pub fn new(
        p: usize,
        size: usize,
    ) -> Self {
        let fact = factorial(p, size);

        let inv_fact = inverse_factorial(p, size);

        Self { p, fact, inv_fact }
    }

    pub fn p(
        &self,
        n: usize,
        k: usize,
    ) -> usize {
        if n < k {
            0
        } else {
            self.fact[n] * self.inv_fact[n - k] % self.p
        }
    }

    pub fn c(
        &self,
        n: usize,
        k: usize,
    ) -> usize {
        self.p(n, k) * self.inv_fact[k] % self.p
    }

    pub fn h(
        &self,
        n: usize,
        k: usize,
    ) -> usize {
        if n == 0 {
            0
        } else {
            self.c(n - 1 + k, k)
        }
    }

    pub fn inv(
        &self,
        n: usize,
    ) -> usize {
        self.fact[n - 1] * self.inv_fact[n] % self.p
    }

    pub fn inv_p(
        &self,
        n: usize,
        k: usize,
    ) -> usize {
        assert!(k <= n);

        self.inv_fact[n] * self.fact[n - k] % self.p
    }

    pub fn inv_c(
        &self,
        n: usize,
        k: usize,
    ) -> usize {
        self.inv_p(n, k) * self.fact[k] % self.p
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let p = 1_000_000_007;

        let f = FactorialTablesFrequentOps::new(p, 100);

        assert_eq!(f.c(5, 2), 10);
    }
}
