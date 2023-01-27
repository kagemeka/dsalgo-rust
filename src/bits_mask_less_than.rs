pub fn bit_mask(less_than: usize) -> usize { (1 << less_than) - 1 }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(bit_mask(4), 0b001111);
    }
}
