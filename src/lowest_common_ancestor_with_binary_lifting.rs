use crate::{
    bit_length_with_count_leading_zeros_usize::bit_length,
    functional_graph_doubling_table::doubling_table,
    tree_bfs_parent_depth::bfs,
};

pub struct LCA {
    pub(crate) depth: Vec<usize>,
    ancestor: Vec<Vec<usize>>,
}

impl LCA {
    pub fn new(
        g: &[Vec<usize>],
        root: usize,
    ) -> Self {
        let (parent, depth) = bfs(g, root);

        let k = bit_length(*depth.iter().max().unwrap());

        let ancestor = doubling_table(&parent, k.max(1));

        Self { ancestor, depth }
    }

    pub fn get(
        &self,
        mut u: usize,
        mut v: usize,
    ) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        let dv = self.depth[v] - self.depth[u];

        for (i, a) in self.ancestor.iter().enumerate() {
            if dv >> i & 1 == 1 {
                v = a[v];
            }
        }

        debug_assert_eq!(self.depth[u], self.depth[v]);

        if u == v {
            return u;
        }

        for a in self.ancestor.iter().rev() {
            let (nu, nv) = (a[u], a[v]);

            if nu != nv {
                u = nu;

                v = nv;
            }
        }

        self.ancestor[0][u]
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
