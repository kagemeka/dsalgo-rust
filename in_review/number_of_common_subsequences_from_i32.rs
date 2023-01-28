use std::ops::*;

pub fn number_of_common_subsequences<N, T: Eq>(
    a: &[T],
    b: &[T],
) -> N
where
    N: From<i32> + Clone + Sub<Output = N> + Add<Output = N>,
{
    let n = a.len();

    let m = b.len();

    if n < m {
        return number_of_common_subsequences::<N, _>(b, a);
    }

    let mut dp: Vec<N> = vec![1.into(); m + 1];

    for i in 0..n {
        for j in (0..m).rev() {
            if a[i] != b[j] {
                dp[j + 1] = dp[j + 1].clone() - dp[j].clone();
            }
        }

        for j in 0..m {
            dp[j + 1] = dp[j + 1].clone() + dp[j].clone();
        }
    }

    dp[m].clone()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
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

        use crate::const_generics_modular_int_i64::Modint;

        const MOD: i64 = 1_000_000_007;

        type Mint = Modint<MOD>;

        for ((a, b), ans) in cases {
            assert_eq!(
                number_of_common_subsequences::<Mint, i32>(&a, &b),
                ans.into()
            );
        }
    }
}
