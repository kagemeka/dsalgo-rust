pub fn count_toposort(adj_bits: &[usize]) -> usize {
    let g = adj_bits;

    let n = g.len();

    let mut dp = vec![0; 1 << n];

    dp[0] = 1;

    for s in 0..1 << n {
        for i in 0..n {
            if s >> i & 1 == 0 && g[i] & s == 0 {
                dp[s | 1 << i] += dp[s];
            }
        }
    }

    *dp.last().unwrap()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (vec![vec![], vec![0, 2], vec![]], 2),
            (vec![vec![1, 3], vec![2], vec![4], vec![4], vec![]], 3),
            (
                vec![
                    vec![1],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
                10461394944000usize,
            ),
        ];

        for (g, ans) in cases {
            let n = g.len();

            let mut gb = vec![0; n];

            for u in 0..n {
                for &v in g[u].iter() {
                    gb[u] |= 1 << v;
                }
            }

            assert_eq!(count_toposort(&gb), ans);
        }
    }
}
