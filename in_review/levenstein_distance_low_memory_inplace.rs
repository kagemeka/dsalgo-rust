pub fn levenstein_dist<T: PartialEq>(
    a: &[T],
    b: &[T],
) -> usize {
    let n: usize = a.len();

    let m: usize = b.len();

    let mut dp: Vec<_> = (0..m + 1).collect();

    for i in 0..n {
        for j in (0..m).rev() {
            dp[j + 1] = if a[i] == b[j] { dp[j] } else { dp[j + 1].min(dp[j]) };
        }

        dp[0] = i + 1;

        for j in 0..m {
            if a[i] != b[j] {
                dp[j + 1] = dp[j + 1].min(dp[j]) + 1;
            }
        }
    }

    dp[m]
}
