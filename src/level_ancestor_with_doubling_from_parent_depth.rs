//! not only tree, but accept all the functional graph like structure
//! with parent and depth for each node.
//! for example, unicyclic tree (each node in the cycle is root) is ok.

use crate::functional_graph_doubling_table::doubling_table;

pub struct LevelAncestor {
    ancestor: Vec<Vec<usize>>,
    pub depth: Vec<usize>,
}

impl LevelAncestor {
    pub fn new(
        parent: Vec<usize>,
        depth: Vec<usize>,
    ) -> Self {
        let k = depth.iter().max().unwrap().next_power_of_two().trailing_zeros()
            as usize;

        let ancestor = doubling_table(&parent, k + 1);

        Self { ancestor, depth }
    }

    pub fn get(
        &self,
        mut u: usize,
        mut k: usize,
    ) -> usize {
        assert!(k <= self.depth[u]);

        for (i, a) in self.ancestor.iter().enumerate() {
            if k >> i & 1 == 0 {
                continue;
            }

            k ^= 1 << i;

            u = a[u];
        }

        debug_assert!(k == 0);

        u
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atcoder_abc267_f() {
        let cases = vec![(
            5,
            vec![(1, 2), (2, 3), (3, 4), (3, 5)],
            vec![(2, 2), (5, 3), (3, 3)],
            vec![4, 1, -1],
        )];

        use crate::{
            tree_bfs_parent_depth::bfs,
            tree_diameter_terminal_nodes::diameter_ends,
        };

        for (n, edges, mut queries, ans) in cases {
            let mut g = vec![vec![]; n];

            for (mut u, mut v) in edges {
                u -= 1;

                v -= 1;

                g[u].push(v);

                g[v].push(u);
            }

            for q in queries.iter_mut() {
                q.0 -= 1;
            }

            let (u, v) = diameter_ends(&g);

            let (p, d) = bfs(&g, u);

            let la_u = LevelAncestor::new(p, d);

            let (p, d) = bfs(&g, v);

            let la_v = LevelAncestor::new(p, d);

            for ((u, k), ans) in queries.into_iter().zip(ans.into_iter()) {
                let res = if k <= la_u.depth[u] {
                    la_u.get(u, k) as isize + 1
                } else if k <= la_v.depth[u] {
                    la_v.get(u, k) as isize + 1
                } else {
                    -1
                };

                assert_eq!(res, ans);
            }
        }
    }
}
