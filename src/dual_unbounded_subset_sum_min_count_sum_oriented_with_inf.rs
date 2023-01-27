pub fn subset_sum(
    inf: usize,
    a: &[usize],
    k: usize,
) -> usize {
    let mut dp = vec![inf; k + 1];

    dp[0] = 0;

    for i in 0..k {
        for &x in a.iter() {
            if i + x > k {
                continue;
            }

            dp[i + x] = dp[i + x].min(dp[i] + 1);
        }
    }

    dp[k]
}
