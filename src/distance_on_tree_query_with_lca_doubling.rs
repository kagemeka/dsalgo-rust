use crate::lowest_common_ancestor_with_binary_lifting::LCA;

/// distance query on tree with LCA binary lifting
/// unweighted.

pub struct TreeDist {
    lca: LCA,
}

impl TreeDist {
    pub fn new(g: &[Vec<usize>]) -> Self { Self { lca: LCA::new(g, 0) } }

    pub fn get(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        let d = &self.lca.depth;

        let lca = self.lca.get(u, v);

        d[u] + d[v] - 2 * d[lca]
    }
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

        for (edges, queries) in cases {
            let n = edges.len() + 1;

            let mut g = vec![vec![]; n];

            for (mut u, mut v) in edges {
                u -= 1;

                v -= 1;

                g[u].push(v);

                g[v].push(u);
            }

            let dist = TreeDist::new(&g);

            for ((mut u, mut v), ans) in queries {
                u -= 1;

                v -= 1;

                assert_eq!(dist.get(u, v) + 1, ans);
            }
        }
    }
}
