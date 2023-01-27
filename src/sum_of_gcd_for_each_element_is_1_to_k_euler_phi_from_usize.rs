use std::ops::*;

use crate::{
    power_multiplicative_semigroup_with_std_ops::power,
    sieve_of_eratosthenes_euler_totient_function_table_direct_usize::phi_table,
};

pub fn sum_of_gcd<T>(
    k: usize,
    n: usize,
) -> T
where
    T: Clone + MulAssign + From<usize> + Mul<Output = T> + AddAssign,
{
    let mut s = 0.into();

    let phi = phi_table(k + 1);

    for i in 1..=k {
        let p: T = phi[i].into();

        s += p * power((k / i).into(), n as u64);
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
