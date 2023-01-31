use crate::{
    factorial_table_from_i32::factorial,
    inverse_factorial_table_from_i32::inverse_factorial,
};

pub struct FactorialTablesFrequentOps<T> {
    pub fact: Vec<T>,
    pub inv_fact: Vec<T>,
}

use std::ops::*;

impl<T> FactorialTablesFrequentOps<T>
where
    T: Clone + Mul<Output = T> + From<i32>,
{
    pub fn new(size: usize) -> Self
    where
        T: Div<Output = T>,
    {
        let fact = factorial(size);

        let inv_fact = inverse_factorial(size);

        Self { fact, inv_fact }
    }

    pub fn p(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        if n < k {
            0.into()
        } else {
            self.fact[n].clone() * self.inv_fact[n - k].clone()
        }
    }

    pub fn c(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        self.p(n, k) * self.inv_fact[k].clone()
    }

    pub fn h(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        if n == 0 {
            0.into()
        } else {
            self.c(n - 1 + k, k)
        }
    }

    pub fn inv(
        &self,
        n: usize,
    ) -> T {
        self.fact[n - 1].clone() * self.inv_fact[n].clone()
    }

    pub fn inv_p(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        assert!(k <= n);

        self.inv_fact[n].clone() * self.fact[n - k].clone()
    }

    pub fn inv_c(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        self.inv_p(n, k) * self.fact[k].clone()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::{
            define_const_modulus_macro::Mod1_000_000_007I64,
            modular_int_with_static_modulus::Modint,
        };

        type Mint = Modint<Mod1_000_000_007I64>;

        let f = FactorialTablesFrequentOps::<Mint>::new(100);

        assert_eq!(f.c(5, 2).0, 10);
    }
}
