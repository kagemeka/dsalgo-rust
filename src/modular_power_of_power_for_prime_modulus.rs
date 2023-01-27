use crate::modular_power_recurse_i64::pow;

/// x^(y^z) mod p
/// fermat's little theorem
/// (x mod p)^(y^z mod (p - 1)) mod p

pub fn pow_of_pow(
    p: i64,
    x: i64,
    y: i64,
    z: i64,
) -> i64 {
    if y == 0 {
        1
    } else if x % p == 0 {
        0
    } else {
        pow(p, x, pow(p - 1, y, z))
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atcoder_abc228_e() {
        const P: i64 = 998_244_353;

        let cases = vec![
            ((2, 2, 2), 16),
            ((3, 14, 15926535), 109718301),
            ((2, 2, P << 1), 0),
            ((2, 0, P << 1), 1),
        ];

        for ((n, k, m), ans) in cases {
            assert_eq!(pow_of_pow(P, m, k, n), ans);
        }
    }
}
