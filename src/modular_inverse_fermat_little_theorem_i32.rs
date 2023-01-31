//! well-known modular inverse algorithms.
//! inverse by Fermat's Little Theorem.
//! for prime modulus.

use crate::modular_power_with_neg_exp_i32::pow;

pub fn modinv(
    p: i32,
    x: i32,
) -> i32 {
    pow(p, x as i64, p as i64 - 2)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i32 = 1_000_000_007;

        const INV: i32 = (MOD + 1) >> 1;

        assert_eq!(modinv(MOD, 2), INV);
    }
}
