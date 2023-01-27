/// edges are directed.
/// similar to bellman ford.
/// but epochs must be specified by user.

pub fn bellman_ford_strict<F>(
    edges: &[(usize, usize)],
    mut f: F,
    epochs: usize,
) where
    F: FnMut(usize, usize),
{
    for _ in 0..epochs {
        for &(u, v) in edges.iter() {
            f(u, v);
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

        for ((n, m, k, s, t, x, edges_origin), ans) in cases {
            let s = s - 1;

            let t = t - 1;

            let x = x - 1;

            let encode = |u: usize, p: usize| -> usize { p * n + u };

            let mut edges = Vec::with_capacity(m * 2 * 4);

            for (u, v) in edges_origin {
                let u = u - 1;

                let v = v - 1;

                let p = if v == x { 1 } else { 0 };

                let q = if u == x { 1 } else { 0 };

                for j in 0..2 {
                    edges.push((encode(u, j), encode(v, j ^ p)));

                    edges.push((encode(v, j), encode(u, j ^ q)));
                }
            }

            let mut dp = vec![0; n * 2];

            let mut ndp = vec![0; n * 2];

            let s = encode(s, 0);

            let t = encode(t, 0);

            dp[s] = 1;

            let mut epochs = 0;

            const MOD: usize = 998_244_353;

            let m = edges.len();

            let mut f = |u: usize, v: usize| {
                ndp[v] += dp[u];

                ndp[v] %= MOD;

                epochs += 1;

                if epochs % m == 0 {
                    std::mem::swap(&mut dp, &mut ndp);

                    for x in ndp.iter_mut() {
                        *x = 0;
                    }
                }
            };

            bellman_ford_strict(&edges, &mut f, k);

            assert_eq!(dp[t], ans);
        }
    }
}
