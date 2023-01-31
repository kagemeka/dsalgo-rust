use crate::extended_euclidean_modular_gcd_inverse_u64_direct::mod_gcd_inv;

/// inverse by Extended Euclidean Algorithm.

pub fn modinv(
    modulus: u64,
    element: u64,
) -> Result<u64, &'static str> {
    let (gcd, inv) = mod_gcd_inv(modulus, element);

    if gcd == 1 {
        Ok(inv)
    } else {
        Err("modulus and element are not coprime")
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((1_000_000_0007, 2), Ok(1_000_000_0008 >> 1)),
            ((8, 3), Ok(3)),
            ((4, 2), Err("modulus and element are not coprime")),
        ];

        for ((m, x), ans) in cases {
            assert_eq!(modinv(m, x), ans);
        }
    }
}
