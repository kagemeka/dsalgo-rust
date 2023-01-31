pub fn pow(
    modulus: i64,
    mut x: i64,
    n: i64,
) -> i64 {
    x %= modulus;

    if n == 0 {
        return 1;
    }

    let mut y = pow(modulus, x, n >> 1);

    y = y * y % modulus;

    if n & 1 == 1 {
        y = y * x % modulus;
    }

    y
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let m = 1_000_000_007;

        let inv_2 = (m + 1) >> 1;

        assert_eq!(pow(m, 2, m - 2), inv_2);
    }
}
