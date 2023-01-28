use std::ops::*;

pub fn power<T>(
    x: T,
    n: u64,
) -> T
where
    T: Clone + MulAssign,
{
    assert!(n > 0);

    if n == 1 {
        return x;
    }

    let mut y = power(x.clone(), n >> 1);

    y *= y.clone();

    if n & 1 == 1 {
        y *= x;
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
