use std::ops::*;

pub fn extgcd<T>(
    a: T,
    b: T,
) -> (T, T, T)
where
    T: Copy
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + From<i32>
        + Ord,
{
    if b == 0.into() {
        return (a, 1.into(), 0.into());
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

        assert_eq!(extgcd(0, 0), (0, 1, 0));
    }
}
