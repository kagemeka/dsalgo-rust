pub fn max_clique(adj_bits: &[usize]) -> usize {
    let g = adj_bits;

    let n = g.len();

    let mut mx = 0;

    for s in 0..1 << n {
        let mut cnt = 0;

        let mut t = s;

        for i in 0..n {
            if s >> i & 1 == 0 {
                continue;
            }

            cnt += 1;

            t &= g[i] | 1 << i;
        }

        if t == s {
            mx = mx.max(cnt);
        }
    }

    mx
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/abc002/tasks/abc002_4
        let cases = vec![
            ((5, vec![(1, 2), (2, 3), (1, 3)]), 3),
            ((5, vec![(1, 2), (2, 3), (3, 4)]), 2),
            (
                (
                    7,
                    vec![
                        (1, 2),
                        (1, 3),
                        (2, 3),
                        (4, 5),
                        (4, 6),
                        (4, 7),
                        (5, 6),
                        (5, 7),
                        (6, 7),
                    ],
                ),
                4,
            ),
            ((12, vec![]), 1),
        ];

        for ((n, edges), ans) in cases {
            let mut g = vec![0; n];

            for (u, v) in edges {
                let u = u - 1;

                let v = v - 1;

                g[u] |= 1 << v;

                g[v] |= 1 << u;
            }

            assert_eq!(max_clique(&g), ans);
        }
    }
}
