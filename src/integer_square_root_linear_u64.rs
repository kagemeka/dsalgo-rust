//! integer square root with linear naive algorithm

pub fn isqrt(n: u64) -> u64 {
    (1..1 << 32).find(|&x| x * x > n).unwrap_or(1 << 32) - 1
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const ISQRT: &[u64] = &[
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10,
        ];

        let n = ISQRT.len();

        for i in 0..n {
            assert_eq!(isqrt(i as u64), ISQRT[i]);
        }
    }
}
