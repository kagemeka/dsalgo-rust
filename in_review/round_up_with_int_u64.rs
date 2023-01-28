pub fn round_up(
    n: u64,
    k: u64,
) -> u64 {
    (n + k - 1) / k * k
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(round_up(100, 15), 105);
    }
}
