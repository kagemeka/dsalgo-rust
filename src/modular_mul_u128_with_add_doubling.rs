/// avoid overflow on u128.
/// pow for addition.
/// under u64 -> it's enough to cast as u128.

pub fn mul_doubling(
    mut a: u128,
    mut b: u128,
    m: u128,
) -> u128 {
    let mut res = 0;

    while b > 0 {
        if b & 1 == 1 {
            res = (res + a) % m;
        }

        a = (a << 1) % m;

        b >>= 1;
    }

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_mul_doubling_128() {
        let a = 1234567890123456789u128;

        let m = 1u128 << 100;

        assert_eq!(mul_doubling(a, a, m), a * a % m,);
    }
}
