use std::ops::*;

use crate::power_multiplicative_semigroup_with_std_ops::power;
// TODO: use fast enumerating power to the n table of size k,
// to make entire complexity O(k|log\log{k} + k(\log{N}/\log{k}).
// for detail: https://37zigen.com/linear-sieve/#i-3
// current: O(k|log\log{k} + k\log{N})
use crate::sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes;

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

    for i in 1..=k {
        d[i] = power((k / i).into(), n as u64);
    }

    for p in enumerate_primes(k as u32 + 1) {
        let p = p as usize;

        for i in (p..=k).step_by(p) {
            let j = i / p;

            d[j] = d[j].clone() - d[i].clone();
        }
    }

    let mut s = 0.into();

    for i in 1..=k {
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
