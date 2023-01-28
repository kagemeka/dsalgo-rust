pub fn extgcd(
    mut a: i64,
    mut b: i64,
) -> (i64, i64, i64) {
    use std::mem::swap;

    let (mut x00, mut x01, mut x10, mut x11) = (1, 0, 0, 1);

    while b != 0 {
        let q = a / b;

        a %= b;

        swap(&mut a, &mut b);

        x00 -= q * x01;

        swap(&mut x00, &mut x01);

        x10 -= q * x11;

        swap(&mut x10, &mut x11);
        // (a, b) = (b, a % b);
        // (x00, x01) = (x01, x00 - q * x01);
        // (x10, x11) = (x11, x10 - q * x11);
    }

    if a >= 0 {
        (a, x00, x10)
    } else {
        (-a, -x00, -x10)
    }
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
