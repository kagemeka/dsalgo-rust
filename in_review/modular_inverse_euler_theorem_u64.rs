// check gcd
// return error if gcd != 1
// or compute inverse with totient function.
// related: carmichael_function.rs
use crate::{
    euler_totient_with_trial_division_u64::phi,
    greatest_common_divisor_euclidean_u64::gcd,
    modular_power_u64::pow,
};

pub fn modinv(
    modulus: u64,
    x: u64,
) -> u64 {
    assert_eq!(gcd(modulus, x), 1);

    pow(modulus, x as u128, phi(modulus) - 1)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: u64 = 1_000_000_007;

        const INV: u64 = (MOD + 1) >> 1;

        assert_eq!(modinv(MOD, 2), INV);
    }
}
