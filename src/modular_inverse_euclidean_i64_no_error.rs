use crate::extended_euclidean_modular_gcd_inverse_i64_with_extgcd::mod_gcd_inv;

pub fn modinv(
    modulus: i64,
    x: i64,
) -> i64 {
    let (g, inv) = mod_gcd_inv(modulus, x);

    assert!(g == 1);

    return inv;
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
