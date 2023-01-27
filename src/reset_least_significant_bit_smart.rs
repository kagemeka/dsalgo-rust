use std::ops::*;

pub fn reset_lsb<T>(n: T) -> T
where
    T: From<i32> + Clone + Sub<Output = T> + BitAnd<Output = T>,
{
    n.clone() & (n - 1.into())
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_reset_least_bit() {
        assert_eq!(reset_lsb(0), 0);

        assert_eq!(reset_lsb(16), 0);

        assert_eq!(reset_lsb(3), 2);

        assert_eq!(reset_lsb(6), 4);

        assert_eq!(reset_lsb(-1), -2);
    }
}
