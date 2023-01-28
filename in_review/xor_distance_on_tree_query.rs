use crate::xor_distance_on_tree_with_dfs_usize::*;

pub struct XorTreeDist(Vec<usize>);

impl XorTreeDist {
    pub fn new(g: &[Vec<(usize, usize)>]) -> Self {
        Self(tree_dfs_xor_dist(&g))
    }

    pub fn calc(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        self.0[u] ^ self.0[v]
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (4, vec![(0, 1, 1), (1, 2, 2), (1, 3, 3)]),
            vec![(0, 1, 1), (2, 3, 1)],
        )];

        for ((n, edges), q) in cases {
            let mut g = vec![vec![]; n];

            for (u, v, w) in edges {
                g[u].push((v, w));

                g[v].push((u, w));
            }

            let f = XorTreeDist::new(&g);

            for (u, v, ans) in q {
                assert_eq!(f.calc(u, v), ans);
            }
        }
    }
}
