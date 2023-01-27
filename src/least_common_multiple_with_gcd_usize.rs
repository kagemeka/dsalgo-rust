use crate::greatest_common_divisor_euclidean::gcd;

pub fn lcm(
    a: usize,
    b: usize,
) -> usize {
    if a == 0 && b == 0 {
        0
    } else {
        a / gcd(a, b) * b
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
    }
}
