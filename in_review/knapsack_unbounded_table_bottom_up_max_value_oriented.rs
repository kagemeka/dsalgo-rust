pub fn knapsack(
    vw: &[(usize, usize)],
    size: usize,
) -> Vec<usize> {
    let mut dp = vec![0; size];

    for i in 0..size {
        for &(v, w) in vw {
            if i + w >= size {
                continue;
            }

            dp[i + w] = dp[i + w].max(dp[i] + v);
        }
    }

    dp
}
