use crate::{
    euler_totient_with_trial_division_i64::phi,
    greatest_common_divisor_euclidean_recurse_i64::gcd,
    modular_power_recurse_i64::pow,
};

pub fn modinv(
    modulus: i64,
    x: i64,
) -> i64 {
    assert_eq!(gcd(modulus, x), 1);

    pow(modulus, x, phi(modulus) - 1)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        const INV: i64 = (MOD + 1) >> 1;

        assert_eq!(modinv(MOD, 2), INV);
    }
}
