use crate::most_significant_bit_with_bit_length_u64::msb;

pub fn log_2(n: u64) -> u64 {
    assert!(n > 0);

    msb(n) as u64
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(10, 3), (8, 3), (1, 0), (2, 1), (3, 1), (4, 2)];

        for (n, ans) in cases {
            assert_eq!(log_2(n), ans);
        }
    }
}
