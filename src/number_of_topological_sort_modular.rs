pub fn count_toposort(
    adj_bits: &[usize],
    modulus: usize,
) -> usize {
    let g = adj_bits;

    let n = g.len();

    let mut dp = vec![0; 1 << n];

    dp[0] = 1;

    for s in 0..1 << n {
        for i in 0..n {
            if s >> i & 1 == 1 || g[i] & s != 0 {
                continue;
            }

            let t = s | 1 << i;

            dp[t] = (dp[t] + dp[s]) % modulus;
        }
    }

    *dp.last().unwrap()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
