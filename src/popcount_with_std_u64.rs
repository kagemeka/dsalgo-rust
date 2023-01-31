/// O(1)

pub fn popcount(n: u64) -> u8 { n.count_ones() as u8 }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [
            (0b1010, 2),
            (0b1100100, 3),
            (0b001010101, 4),
            (0b11111, 5),
            (0b00000, 0),
        ];

        for (n, ans) in cases {
            assert_eq!(popcount(n), ans);
        }
    }
}
