pub fn path_count(
    g: &[Vec<usize>],
    src: usize,
    modulus: usize,
) -> Vec<usize> {
    let n = g.len();

    let mut f = vec![0; n];

    f[src] = 1;

    let mut que = std::collections::VecDeque::new();

    que.push_back(src);

    let mut to_que = vec![false; n];

    to_que[src] = true;

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            f[v] += f[u];

            f[v] %= modulus;

            if to_que[v] {
                continue;
            }

            to_que[v] = true;

            que.push_back(v);
        }
    }

    f
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

            let s = encode(0, s, 0);

            let t = encode(k, t, 0);

            const MOD: usize = 998_244_353;

            let cnt = path_count(&g, s, MOD);

            assert_eq!(cnt[t], ans);
        }
    }
}
