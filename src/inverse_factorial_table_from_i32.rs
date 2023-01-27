use std::ops::*;

use crate::{
    cumulative_product_vec_with_std_mul::cumprod,
    factorial_table_from_i32::factorial,
};

pub fn inverse_factorial<T>(size: usize) -> Vec<T>
where
    T: Mul<Output = T> + Div<Output = T> + From<i32> + Clone,
{
    if size == 0 {
        return vec![];
    }

    let mut inv_fact =
        (0..size as i32).rev().map(|i| (i + 1).into()).collect::<Vec<T>>();

    inv_fact[0] = T::from(1) / factorial::<T>(size)[size - 1].clone();

    inv_fact = cumprod(inv_fact);

    inv_fact.reverse();

    inv_fact
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

        let res = inverse_factorial::<Mint>(20)
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<i64>>();

        assert_eq!(
            res,
            [
                1, 1, 500000004, 166666668, 41666667, 808333339, 301388891,
                900198419, 487524805, 831947206, 283194722, 571199524,
                380933296, 490841026, 320774361, 821384963, 738836565,
                514049213, 639669405, 402087866
            ],
        );
    }
}
