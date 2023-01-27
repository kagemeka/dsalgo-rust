use std::ops::*;

use crate::inverse_factorial_table_from_i32::inverse_factorial;

pub fn n_choose<T>(
    n: usize,
    size: usize,
) -> Vec<T>
where
    T: Mul<Output = T> + Div<Output = T> + From<i32> + Clone,
{
    assert!(size > 0);

    let mut f: Vec<T> = vec![0.into(); size];

    f[0] = 1.into();

    for i in 1..size {
        f[i] = f[i - 1].clone() * ((n + 1 - i) as i32).into();
    }

    for (f, inv) in f.iter_mut().zip(inverse_factorial::<T>(size).into_iter()) {
        *f = f.clone() * inv;
    }

    f
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

        let res = n_choose::<Mint>(1_000_000_000, 20)
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<i64>>();

        assert_eq!(
            res,
            [
                1, 1000000000, 28, 999999923, 210, 999999545, 924, 999998291,
                3003, 999995002, 8008, 999987631, 18564, 999972875, 38760,
                999945743, 74613, 999899060, 134596, 999822907
            ]
        );
    }
}
