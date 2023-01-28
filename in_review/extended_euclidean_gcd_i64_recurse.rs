pub fn extgcd(
    a: i64,
    b: i64,
) -> (i64, i64, i64) {
    if b == 0 {
        return if a < 0 { (-a, -1, 0) } else { (a, 1, 0) };
    }

    let (g, s, t) = extgcd(b, a % b);

    (g, t, s - a / b * t)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_extgcd() {
        assert_eq!(extgcd(-30, 111), (3, 11, 3));

        assert_eq!(extgcd(111, 30), (3, 3, -11));

        assert_eq!(extgcd(0, 0), (0, 1, 0));
    }
}
