//! linear with subtraction
//! reference
//! https://en.wikipedia.org/wiki/Integer_square_root

pub fn isqrt(n: u64) -> u64 {
    let mut a = 5 * n;

    let mut b = 5;

    while a >= b {
        a -= b;

        b += 10;
    }

    b / 10
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
