pub fn pow(
    m: usize,
    x: usize,
    exp: usize,
) -> usize {
    if exp == 0 {
        return 1;
    }

    let mut y = pow(m, x, exp >> 1);

    y *= y;

    y %= m;

    if exp & 1 == 1 {
        y *= x;

        y %= m;
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
