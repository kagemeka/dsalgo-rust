pub fn bit_mask(
    l: usize,
    r: usize,
) -> usize {
    (1 << (r - l)) - 1 << l
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(bit_mask(1, 4), 0b01110);
    }
}
