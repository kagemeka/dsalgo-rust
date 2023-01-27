pub fn reset_lsb(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n & !(1 << n.trailing_zeros())
    }
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
    }
}
