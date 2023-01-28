pub fn add(
    a: u64,
    b: u64,
) -> u64 {
    (a ^ b) + ((a & b) << 1)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(add(10, 5), 15);
    }
}
