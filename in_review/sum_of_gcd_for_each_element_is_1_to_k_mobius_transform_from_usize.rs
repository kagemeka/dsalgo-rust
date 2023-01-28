use std::ops::*;

use crate::power_multiplicative_semigroup_with_std_ops::power;

pub fn sum_of_gcd<T>(
    k: usize,
    n: usize,
) -> T
where
    T: Clone
        + MulAssign
        + From<usize>
        + Mul<Output = T>
        + AddAssign
        + Sub<Output = T>,
{
    let mut d: Vec<T> = vec![0.into(); k + 1];

    let mut s = 0.into();

    for i in (1..=k).rev() {
        d[i] = power((k / i).into(), n as u64);

        for j in (i << 1..=k).step_by(i) {
            d[i] = d[i].clone() - d[j].clone();
        }

        s += d[i].clone() * i.into();
    }

    s
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

        let cases = [(2, 3, 9), (200, 3, 10813692)];

        for (k, n, ans) in cases {
            assert_eq!(sum_of_gcd::<Mint>(k, n).0, ans);
        }
    }
}
