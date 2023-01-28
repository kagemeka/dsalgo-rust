pub fn pow(
    p: usize,
    x: usize,
    exp: isize,
) -> usize {
    if exp == 0 {
        return 1;
    }

    if exp < 0 {
        return pow(p, pow(p, x, p as isize - 2), -exp);
    }

    let mut y = pow(p, x, exp >> 1);

    y *= y;

    y %= p;

    if exp & 1 == 1 {
        y *= x;

        y %= p;
    }

    y
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let p = 1_000_000_007;

        dbg!(pow(p, 2, -1));

        assert_eq!(pow(p, 2, -1), pow(p, 2, p as isize - 2));
    }
}
