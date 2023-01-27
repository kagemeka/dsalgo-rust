use std::ops::*;

pub fn power<T>(
    mut x: T,
    mut n: u64,
) -> T
where
    T: Clone + MulAssign,
{
    assert!(n > 0);

    let mut y = x.clone();

    n -= 1;

    while n > 0 {
        if n & 1 == 1 {
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
