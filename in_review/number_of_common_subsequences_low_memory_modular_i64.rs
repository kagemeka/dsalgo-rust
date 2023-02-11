pub fn number_of_common_subsequences<T: Eq>(
    modulus: i64,
    a: &[T],
    b: &[T],
) -> i64 {
    let n = a.len();

    let m = b.len();

    let mut dp = vec![1; m + 1];

    for i in 0..n {
        for j in (0..m).rev() {
            if a[i] != b[j] {
                dp[j + 1] -= dp[j];
            }
        }

        for j in 0..m {
            dp[j + 1] += dp[j];

            dp[j + 1] %= modulus;
        }
    }

    (dp[m] + modulus) % modulus
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let m = 1_000_000_007;

        let cases = vec![
            ((vec![1, 3], vec![3, 1]), 3),
            ((vec![1, 1], vec![1, 1]), 6),
            ((vec![3, 4, 5, 6], vec![3, 4, 5, 6]), 16),
            (
                (
                    vec![9, 6, 5, 7, 5, 9, 8, 5, 6, 7],
                    vec![8, 6, 8, 5, 5, 7, 9, 9, 7],
                ),
                191,
            ),
            (
                (
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1,
                    ],
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1,
                    ],
                ),
                846527861,
            ),
        ];

        for ((a, b), ans) in cases {
            assert_eq!(number_of_common_subsequences(m, &a, &b), ans);
        }
    }
}
