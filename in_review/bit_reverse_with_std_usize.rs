pub fn bit_reverse(n: usize) -> usize { n.reverse_bits() }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(bit_reverse(0xffffffff0000f000), 0x000f0000ffffffff);
    }
}
