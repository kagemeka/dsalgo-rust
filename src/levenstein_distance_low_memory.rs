pub fn levenstein_dist<T: PartialEq>(
    a: &[T],
    b: &[T],
) -> usize {
    let n: usize = a.len();

    let m: usize = b.len();

    let mut dp: Vec<_> = (0..m + 1).collect();

    for i in 0..n {
        let mut ndp = dp.clone();

        ndp[0] = i + 1;

        for j in 0..m {
            ndp[j + 1] = if a[i] == b[j] {
                dp[j]
            } else {
                dp[j].min(dp[j + 1]).min(ndp[j]) + 1
            };
        }

        dp = ndp;
    }

    dp[m]
}
