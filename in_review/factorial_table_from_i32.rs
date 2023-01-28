use std::ops::*;

use crate::cumulative_product_vec_with_std_mul::cumprod;

pub fn factorial<T>(size: usize) -> Vec<T>
where
    T: Mul<Output = T> + From<i32> + Clone,
{
    if size == 0 {
        return vec![];
    }

    let mut fact = (0..size as i32).map(|i| i.into()).collect::<Vec<T>>();

    fact[0] = 1.into();

    cumprod(fact)
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

        let res = factorial::<Mint>(20)
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<i64>>();

        assert_eq!(
            res,
            [
                1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800,
                39916800, 479001600, 227020758, 178290591, 674358851,
                789741546, 425606191, 660911389, 557316307
            ],
        );
    }
}
