pub fn popcount(mut n: usize) -> usize {
    let mut cnt = 0;

    while n > 0 {
        cnt += 1;

        n &= n - 1;
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
