use std::ops::*;

pub fn power<T, N>(
    mut x: T,
    mut n: N,
) -> T
where
    T: Clone + MulAssign,
    N: From<i32>
        + Ord
        + SubAssign
        + ShrAssign<usize>
        + BitAnd<Output = N>
        + Copy,
{
    let zero = 0.into();

    let one = 1.into();

    assert!(n > zero);

    let mut y = x.clone();

    n -= one;

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
    }
}
