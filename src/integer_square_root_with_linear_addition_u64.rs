//! integer square root with addition

pub fn isqrt(n: u64) -> u64 {
    let mut x = 0;

    let mut x2 = 0; // x^2
    let mut delta = 1; // x2 + delta = (x + 1)^2
    while x2 <= n {
        x += 1;

        x2 += delta;

        delta += 2;
    }

    x - 1
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
