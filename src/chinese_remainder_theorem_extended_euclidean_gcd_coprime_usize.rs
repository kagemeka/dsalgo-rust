use crate::chinese_remainder_theorem_extended_euclidean_gcd_safe_usize::*;

pub fn crt_coprime(
    m0: usize,
    r0: usize,
    m1: usize,
    r1: usize,
) -> (usize, usize) {
    safe_crt(m0, r0, m1, r1).unwrap()
}
