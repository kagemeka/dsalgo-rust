//! level ancestor on tree with doubing (or binary lifting)

use crate::{
    functional_graph_doubling_table::doubling_table, tree_bfs_parent_depth::bfs,
};

pub struct LevelAncestor {
    ancestor: Vec<Vec<usize>>,
    pub depth: Vec<usize>,
}

impl LevelAncestor {
    pub fn new(g: &[Vec<usize>], root: usize) -> Self {
        let (parent, depth) = bfs(g, root);

        let k = depth.iter().max().unwrap().next_power_of_two().trailing_zeros()
            as usize;

        let ancestor = doubling_table(&parent, k + 1);

        Self { ancestor, depth }
    }

    pub fn get(&self, mut u: usize, mut k: usize) -> usize {
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

        use crate::tree_diameter_terminal_nodes::diameter_ends;

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

            let la_u = LevelAncestor::new(&g, u);

            let la_v = LevelAncestor::new(&g, v);

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
