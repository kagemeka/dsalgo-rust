pub fn get(
    s: &str,
    i: usize,
) -> char {
    s.chars().nth(i).unwrap()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let s = "あいうえおabcdefg";

        assert_eq!(get(&s, 1), 'い');

        assert_eq!(get(&s, 6), 'b');
    }
}
