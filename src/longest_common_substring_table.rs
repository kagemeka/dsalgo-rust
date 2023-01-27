pub fn longest_common_substring<T: Eq>(
    a: &[T],
    b: &[T],
) -> Vec<Vec<usize>> {
    let n = a.len();

    let m = b.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            }
        }
    }

    dp
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ("xy", vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 2]]),
            (
                "ababa",
                vec![
                    vec![0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 1, 0, 1],
                    vec![0, 0, 2, 0, 2, 0],
                    vec![0, 1, 0, 3, 0, 3],
                    vec![0, 0, 2, 0, 4, 0],
                    vec![0, 1, 0, 3, 0, 5],
                ],
            ),
            (
                "strangeorange",
                vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 3, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 4, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 5],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 10, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 11, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 12, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 13],
                ],
            ),
        ];

        for (s, ans) in cases {
            let s = s.as_bytes();

            let dp = longest_common_substring(s, s);

            assert_eq!(dp, ans);
        }
    }
}
