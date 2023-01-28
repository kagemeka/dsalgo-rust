use crate::popcount_table_const_8_bit_usize::POPCOUNT;

pub fn popcount(mut n: usize) -> usize {
    const MASK: usize = 0xff;

    let mut cnt = 0;

    while n > 0 {
        cnt += POPCOUNT[n & MASK];

        n >>= 8;
    }

    cnt
}

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
