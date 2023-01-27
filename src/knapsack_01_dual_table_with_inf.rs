pub fn knapsack_dual(
    inf: usize,
    vw: &[(usize, usize)],
) -> Vec<usize> {
    let size = vw.iter().map(|x| x.0).sum::<usize>() + 1;

    let mut dp = vec![inf; size];

    dp[0] = 0;

    for &(v, w) in vw {
        for i in (v..size).rev() {
            dp[i] = dp[i].min(dp[i - v] + w);
        }
    }

    dp
}
