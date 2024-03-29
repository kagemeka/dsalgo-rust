use std::ops::*;

pub fn power<T, N>(mut x: T, mut n: N) -> T
where
    T: Clone + MulAssign + From<i32>,
    N: From<i32>
        + Ord
        + SubAssign
        + ShrAssign<usize>
        + BitAnd<Output = N>
        + Copy,
{
    let zero = 0.into();

    let one = 1.into();

    assert!(n >= zero);

    let mut y = 1.into();

    while n > zero {
        if n & one == one {
            y *= x.clone();
        }

        x *= x.clone();

        n >>= 1;
    }

    y
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(power(2, 10), 1024);

        assert_eq!(power(2, 0), 1);
    }
}
