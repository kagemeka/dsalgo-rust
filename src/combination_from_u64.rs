use std::ops::*;

use crate::{
    factorial_table_from_u64::factorial_table,
    inverse_factorial_table_from_u64::inverse_factorial_table,
};

pub struct Combination<T> {
    fact: Vec<T>,
    inv_fact: Vec<T>,
}

impl<T> Combination<T>
where
    T: Mul<Output = T> + From<u64> + Clone,
{
    pub fn new(size: usize) -> Self
    where
        T: Div<Output = T>,
    {
        Self {
            fact: factorial_table::<T>(size),
            inv_fact: inverse_factorial_table::<T>(size),
        }
    }

    pub fn calc(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        if k > n {
            return 0.into();
        }

        self.fact[n].clone()
            * self.inv_fact[n - k].clone()
            * self.inv_fact[k].clone()
    }

    pub fn inv(
        &self,
        n: usize,
        k: usize,
    ) -> T {
        assert!(k <= n); // (n, k) := 0 if k > n, so the inverse is undefined.
        self.inv_fact[n].clone()
            * self.fact[k].clone()
            * self.fact[n - k].clone()
    }
}

// #[cfg(test)]
mod tests {

    #[test]

    fn test() {
        use super::*;
        use crate::{
            default_static_modular_arithmetic::Modular1_000_000_007,
            modular_int_with_arithmetic::Modint,
        };

        type Mint = Modint<u32, Modular1_000_000_007>;

        let choose = Combination::<Mint>::new(100);

        assert_eq!(choose.calc(5, 2), 10.into());
    }
}
