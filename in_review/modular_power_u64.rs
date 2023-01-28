use crate::power::pow_monoid;

pub fn pow(
    m: u64,
    base: u128,
    exp: u64,
) -> u64 {
    let modulus = m as u128;

    pow_monoid(&|x, y| x * y % modulus, &|| 1, base % modulus, exp) as u64
}
