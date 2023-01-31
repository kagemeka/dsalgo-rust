pub use crate::union_find_low_memory_minimal::UnionFind;

pub fn lca(
    g: &[Vec<usize>],
    queries: &[(usize, usize)],
    root: usize,
) -> Vec<usize> {
    let n = g.len();

    let mut q = vec![vec![]; n];

    for (i, &(u, v)) in queries.iter().enumerate() {
        q[u].push((v, i));

        q[v].push((u, i));
    }

    let mut lca = vec![n; queries.len()];

    let mut top = vec![n; n];

    let mut uf = UnionFind::new(n);

    fn dfs(
        g: &[Vec<usize>],
        q: &[Vec<(usize, usize)>],
        uf: &mut UnionFind,
        top: &mut [usize],
        lca: &mut [usize],
        u: usize,
    ) {
        let n = g.len();

        top[u] = u;

        for &v in g[u].iter() {
            if top[v] != n {
                continue;
            }

            dfs(g, q, uf, top, lca, v);

            uf.unite(u, v);

            top[uf.root(u)] = u;
        }

        for &(v, i) in q[u].iter() {
            if top[v] == n {
                continue;
            }

            lca[i] = top[uf.root(v)];
        }
    }

    dfs(g, &q, &mut uf, &mut top, &mut lca, root);

    lca
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atcoder_abc014_d() {
        let cases = vec![
            (
                vec![(1, 2), (1, 3), (1, 4), (4, 5)],
                vec![((2, 3), 3), ((2, 5), 4), ((2, 4), 3)],
            ),
            (
                vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)],
                vec![((1, 3), 3), ((1, 4), 4), ((1, 5), 5), ((1, 6), 6)],
            ),
            (
                vec![(3, 1), (2, 1), (2, 4), (2, 5), (3, 6), (3, 7)],
                vec![
                    ((4, 5), 3),
                    ((1, 6), 3),
                    ((5, 6), 5),
                    ((4, 7), 5),
                    ((5, 3), 4),
                ],
            ),
        ];

        use crate::tree_bfs_parent_depth::bfs;

        for (edges, queries) in cases {
            let n = edges.len() + 1;

            let mut g = vec![vec![]; n];

            for (mut u, mut v) in edges {
                u -= 1;

                v -= 1;

                g[u].push(v);

                g[v].push(u);
            }

            let qs: Vec<_> =
                queries.iter().map(|((u, v), _)| (u - 1, v - 1)).collect();

            let lca = lca(&g, &qs, 0);

            let (_, depth) = bfs(&g, 0);

            for i in 0..queries.len() {
                let (u, v) = qs[i];

                let ans = queries[i].1;

                let l = lca[i];

                let d = depth[u] + depth[v] - 2 * depth[l];

                assert_eq!(d + 1, ans);
            }
        }
    }
}
