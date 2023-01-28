use std::ops::*;

pub fn double_factorial<T>(size: usize) -> Vec<T>
where
    T: Mul<Output = T> + From<i32> + Clone,
{
    if size == 0 {
        return vec![];
    }

    let mut fact = (0..size as i32).map(|i| i.into()).collect::<Vec<T>>();

    fact[0] = 1.into();

    for i in 2..size {
        fact[i] = fact[i].clone() * fact[i - 2].clone();
    }

    fact
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

        let res = double_factorial::<Mint>(20)
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<i64>>();

        assert_eq!(
            res,
            [
                1, 1, 2, 3, 8, 15, 48, 105, 384, 945, 3840, 10395, 46080,
                135135, 645120, 2027025, 10321920, 34459425, 185794560,
                654729075,
            ]
        );
    }
}
