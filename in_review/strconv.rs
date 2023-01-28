//! module name is following Go's package.

pub fn str_to_byte_vec(s: &str) -> Vec<u8> { s.bytes().collect::<Vec<_>>() }

pub fn str_to_char_vec(s: &str) -> Vec<char> { s.chars().collect::<Vec<_>>() }

pub fn char_slice_to_string(s: &[char]) -> String { s.iter().collect() }

pub fn chars_to_byte_vec<I: Iterator<Item = char>>(
    chars: I,
    offset: u8,
) -> Vec<u8> {
    chars.map(|c| c as u8 - offset).collect::<Vec<_>>()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test_str_to_byte_vec() {
        use super::*;

        let s = "abc";

        assert_eq!(str_to_byte_vec(s), vec![b'a', b'b', b'c'],);
    }

    #[test]

    fn test_str_to_char_vec() {
        use super::*;

        let s = "abc";

        assert_eq!(str_to_char_vec(s), vec!['a', 'b', 'c']);
    }

    #[test]

    fn test_char_slice_to_string() {
        use super::*;

        let s = vec!['a', 'b', 'c'];

        assert_eq!(char_slice_to_string(&s), "abc");
    }

    #[test]

    fn test_chars_to_byte_vec() {
        use super::*;

        let s = "abc";

        assert_eq!(chars_to_byte_vec(s.chars(), b'a'), vec![0, 1, 2]);
    }
}
