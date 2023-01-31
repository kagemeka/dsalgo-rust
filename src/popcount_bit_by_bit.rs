/// O(\log{N})

pub fn popcount(mut n: u64) -> u8 {
    let mut cnt = 0;

    while n > 0 {
        cnt += (n & 1) as u8;

        n >>= 1
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [(0b001010101, 4), (0b11111, 5), (0b00000, 0)];

        for (n, ans) in cases {
            assert_eq!(popcount(n), ans);
        }
    }
}
