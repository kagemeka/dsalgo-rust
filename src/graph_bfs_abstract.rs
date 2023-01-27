pub fn graph_bfs<F>(
    g: &[Vec<usize>],
    mut f: F,
    src: usize,
) where
    F: FnMut(usize, usize) -> bool,
{
    let mut que = std::collections::VecDeque::new();

    que.push_back(src);

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if f(u, v) {
                que.push_back(v);
            }
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atcoder_abc244_e() {
        let cases = vec![
            ((4, 4, 4, 1, 3, 2, vec![(1, 2), (2, 3), (3, 4), (1, 4)]), 4),
            (
                (
                    6,
                    5,
                    10,
                    1,
                    2,
                    3,
                    vec![(2, 3), (2, 4), (4, 6), (3, 6), (1, 5)],
                ),
                0,
            ),
            (
                (
                    10,
                    15,
                    20,
                    4,
                    4,
                    6,
                    vec![
                        (2, 6),
                        (2, 7),
                        (5, 7),
                        (4, 5),
                        (2, 4),
                        (3, 7),
                        (1, 7),
                        (1, 4),
                        (2, 9),
                        (5, 10),
                        (1, 3),
                        (7, 8),
                        (7, 9),
                        (1, 6),
                        (1, 2),
                    ],
                ),
                952504739,
            ),
        ];

        for ((n, m, k, s, t, x, edges), ans) in cases {
            let s = s - 1;

            let t = t - 1;

            let x = x - 1;

            const B: usize = 1 << 11;

            let encode =
                |i: usize, u: usize, p: usize| -> usize { (p * n + u) * B + i };

            const N: usize = 1 << 23;

            let mut g = vec![vec![]; N];

            for (u, v) in edges {
                let u = u - 1;

                let v = v - 1;

                let p = if v == x { 1 } else { 0 };

                let q = if u == x { 1 } else { 0 };

                for i in 0..k {
                    for j in 0..2 {
                        let x = encode(i, u, j);

                        let y = encode(i + 1, v, j ^ p);

                        g[x].push(y);

                        let x = encode(i, v, j);

                        let y = encode(i + 1, u, j ^ q);

                        g[x].push(y);
                    }
                }
            }

            let mut dp = vec![0; N];

            let s = encode(0, s, 0);

            dp[s] = 1;

            const MOD: usize = 998_244_353;

            let mut to_que = vec![false; N];

            to_que[s] = true;

            let mut f = |u: usize, v: usize| -> bool {
                dp[v] += dp[u];

                dp[v] %= MOD;

                if to_que[v] {
                    false
                } else {
                    to_que[v] = true;

                    true
                }
            };

            graph_bfs(&g, &mut f, s);

            let t = encode(k, t, 0);

            assert_eq!(dp[t], ans);
        }
    }
}
