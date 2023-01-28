use crate::greatest_common_divisor_euclidean_recurse_i64::gcd;

pub fn lcm(
    a: i64,
    b: i64,
) -> i64 {
    if a == 0 && b == 0 {
        0
    } else {
        (a / gcd(a, b) * b).abs()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(lcm(0, 0), 0);

        assert_eq!(lcm(1, 0), 0);

        assert_eq!(lcm(12, 18), 36);

        assert_eq!(lcm(6, 8), 24);

        assert_eq!(lcm(0, -1), 0);

        assert_eq!(lcm(-1, 2), 2);
    }
}
