use crate::{
    modular_inverse_euclidean_i64_no_error::modinv,
    power_monoid::pow_monoid,
};

pub fn pow(
    m: i32,
    mut base: i64,
    mut exp: i64,
) -> i32 {
    let modulus = m as i64;

    if exp < 0 {
        base = modinv(modulus, base);

        exp *= -1;
    }

    pow_monoid(&|x, y| x * y % modulus, &|| 1, base % modulus, exp as u64)
        as i32
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(pow(1_000_000_007, 2, -1), 1_000_000_008 >> 1);
    }
}
