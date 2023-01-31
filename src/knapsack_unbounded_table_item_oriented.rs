pub fn knapsack(
    vw: &[(usize, usize)],
    size: usize,
) -> Vec<usize> {
    let mut dp = vec![0; size];

    for &(v, w) in vw {
        for i in w..size {
            dp[i] = dp[i].max(dp[i - w] + v);
        }
    }

    dp
}
