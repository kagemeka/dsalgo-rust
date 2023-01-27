use crate::extended_euclidean_gcd_i64::extgcd;

pub fn mod_gcd_inv(
    modulus: i64,
    n: i64,
) -> (i64, i64) {
    let (g, mut x, _) = extgcd(n, modulus);

    let u = modulus / g;

    if x < 0 {
        x += u;
    }

    debug_assert!(0 <= x && x <= u);

    (g, x)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_mod_gcd_inv() {
        // euclidean_mod_gcd_inv(10, 0); // runtime error.
        assert_eq!(mod_gcd_inv(5, 2), (1, 3));

        assert_eq!(mod_gcd_inv(18, 12), (6, 2));

        assert_eq!(mod_gcd_inv(111, 30), (3, 26));
        // gcd(111, 30) = 3
        // 111 / 3 = 37, 30 / 3 = 10, 10^{-1} \equiv 26 \mod 37
    }
}
