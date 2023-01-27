pub fn gcd(
    mut a: u64,
    mut b: u64,
) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [
            ((100, 3), 1),
            ((1, 0), 1),
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
