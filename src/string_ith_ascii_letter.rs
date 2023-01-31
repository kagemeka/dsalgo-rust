// split by not char, but ascii chunks
pub fn get(
    s: &str,
    i: usize,
) -> u8 {
    s.as_bytes()[i]
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let s = "あいうえおabcdefg";

        assert_eq!(get(&s, 0), 227);

        assert_eq!(get(&s, 1), 129);

        assert_eq!(get(&s, 2), 130);

        assert_eq!(get(&s, 16), 98); // b
    }
}
