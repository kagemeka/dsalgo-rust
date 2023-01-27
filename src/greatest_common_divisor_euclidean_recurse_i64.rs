pub fn gcd(
    a: i64,
    b: i64,
) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [
            ((100, -3), 1),
            ((-1, 0), 1),
            ((0, 0), 0),
            ((5, 7), 1),
            ((5, 0), 5),
            ((4, 0), 4),
            ((0, 4), 4),
            ((9, 6), 3),
        ];

        for ((a, b), ans) in cases {
            assert_eq!(gcd(a, b), ans);
        }
    }
}
