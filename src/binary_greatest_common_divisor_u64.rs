pub fn binary_gcd(
    mut a: u64,
    mut b: u64,
) -> u64 {
    if a == 0 || b == 0 {
        return a + b;
    }

    let i = a.trailing_zeros();

    a >>= i;

    let j = b.trailing_zeros();

    b >>= j;

    while a != b {
        if a < b {
            (a, b) = (b, a);
        }

        a -= b;

        a >>= a.trailing_zeros();
    }

    a << i.min(j)
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
            assert_eq!(binary_gcd(a, b), ans);
        }
    }
}
