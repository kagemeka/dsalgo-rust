use crate::power::pow_monoid;

/// pow for u32
/// why not only u64?
/// because it's expensive to cast as u128.

pub fn pow(
    m: u32,
    base: u64,
    exp: u64,
) -> u32 {
    let modulus = m as u64;

    pow_monoid(&|x, y| x * y % modulus, &|| 1, base % modulus, exp) as u32
}
