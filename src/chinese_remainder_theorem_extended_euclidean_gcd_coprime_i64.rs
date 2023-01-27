use crate::chinese_remainder_theorem_extended_euclidean_gcd_safe_i64::*;

pub fn crt_coprime(
    m0: i64,
    r0: i64,
    m1: i64,
    r1: i64,
) -> (i64, i64) {
    safe_crt(m0, r0, m1, r1).unwrap()
}
