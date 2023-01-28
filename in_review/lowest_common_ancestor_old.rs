//! lowest common ancestor

pub mod tree {

    //! LCA for undirected tree.

    use crate::{
        bit_length_with_count_leading_zeros_u64::bit_length,
        tree_bfs_depth_with_abstract::tree_depths,
        tree_bfs_parent_with_abstract::tree_parents,
    };

    pub struct Doubling {
        a: Vec<Vec<usize>>, // ancestor
        d: Vec<usize>,      // depth
    }

    // TODO: split off doubling part as module.
    impl Doubling {
        pub fn new(e: &[(usize, usize)]) -> Self {
            const R: usize = 0;

            let n = e.len() + 1;

            let d = tree_depths(&e, R);

            let k = bit_length(*d.iter().max().unwrap() as u64).max(1) as usize;

            let mut a = vec![vec![n; n]; k];

            // TODO: make tree parents return Vec<usize>
            // instead of Vec<Option<usize>> (p[root] = root)
            let mut p = tree_parents(&e, R);

            p[R] = Some(R);

            a[0] = p.iter().map(|&v| v.unwrap()).collect();

            for i in 0..k - 1 {
                for j in 0..n {
                    a[i + 1][j] = a[i][a[i][j]];
                }
            }

            Self { a, d }
        }

        pub fn get(
            &self,
            mut u: usize,
            mut v: usize,
        ) -> usize {
            if self.d[u] > self.d[v] {
                std::mem::swap(&mut u, &mut v);
            }

            let d = self.d[v] - self.d[u];

            for i in 0..bit_length(d as u64) as usize {
                if d >> i & 1 == 1 {
                    v = self.a[i][v];
                }
            }

            if v == u {
                return u;
            }

            for a in self.a.iter().rev() {
                let (nu, nv) = (a[u], a[v]);

                if nu != nv {
                    u = nu;

                    v = nv;
                }
            }

            self.a[0][u]
        }
    }

    use crate::{
        tree_edges_to_graph::tree_edges_to_graph,
        union_find_low_memory_with_trait::*,
        union_find_traits::*,
    };

    /// tarjan's offline algorithm

    pub fn tarjan(
        e: &[(usize, usize)],
        qs: &[(usize, usize)],
    ) -> Vec<usize> {
        fn dfs(
            g: &Vec<Vec<usize>>,
            q: &Vec<Vec<(usize, usize)>>,
            visited: &mut Vec<bool>,
            uf: &mut UnionFind,
            a: &mut Vec<usize>,
            lca: &mut Vec<usize>,
            u: usize,
        ) {
            visited[u] = true;

            a[u] = u;

            for &v in g[u].iter() {
                if visited[v] {
                    continue;
                }

                dfs(g, q, visited, uf, a, lca, v);

                uf.unite(u, v);

                a[uf.root(u)] = u;
            }

            q[u].iter().filter(|&&(v, _)| visited[v]).for_each(|&(v, i)| {
                lca[i] = a[uf.root(v)];
            });
        }

        let n = e.len() + 1;

        let graph = tree_edges_to_graph(e);

        let mut q = vec![vec![]; n];

        for (i, &(u, v)) in qs.iter().enumerate() {
            q[u].push((v, i));

            q[v].push((u, i));
        }

        let mut visited = vec![false; n];

        let mut uf = UnionFind::new(n);

        let mut ancestor = vec![n; n];

        let mut lca = vec![n; qs.len()];

        dfs(&graph, &q, &mut visited, &mut uf, &mut ancestor, &mut lca, 0);

        lca
    }

    use crate::heavly_light_decomposition::heavy_light_decompose;

    pub struct LCAHLD {
        p: Vec<Option<usize>>, // parents
        d: Vec<usize>,         // depths
        r: Vec<usize>,         // roots
    }

    impl LCAHLD {
        pub fn new(
            tree_edges: &[(usize, usize)],
            root: usize,
        ) -> Self {
            Self {
                p: tree_parents(tree_edges, root),
                d: tree_depths(tree_edges, root),
                r: heavy_light_decompose(tree_edges, root),
            }
        }

        pub fn get(
            &self,
            mut u: usize,
            mut v: usize,
        ) -> usize {
            use std::mem::swap;

            while self.r[u] != self.r[v] {
                if self.d[self.r[u]] > self.d[self.r[v]] {
                    swap(&mut u, &mut v);
                }

                v = self.p[self.r[v]].unwrap();
            }

            if self.d[u] <= self.d[v] {
                u
            } else {
                v
            }
        }
    }

    use crate::{
        ett::{
            first_positions,
            tour_nodes,
        },
        query::RangeMinimumQuery,
    };

    /// with euler tour and static range minimum query.

    pub struct EulerTourRMQ<Q> {
        first_pos: Vec<usize>,
        rmq: Q,
    }

    impl<Q> EulerTourRMQ<Q> {
        pub fn new(
            tree_edges: &[(usize, usize)],
            root: usize,
        ) -> Self
        where
            Q: std::iter::FromIterator<(usize, usize)>,
        {
            let tour_nodes = tour_nodes(tree_edges, root);

            let depth = tree_depths(tree_edges, root);

            let first_pos = first_positions(&tour_nodes);

            let depth =
                tour_nodes.iter().map(|&u| depth[u]).collect::<Vec<_>>();

            let rmq =
                Q::from_iter(depth.into_iter().zip(tour_nodes.into_iter()));

            Self { first_pos, rmq }
        }

        pub fn get(
            &mut self,
            u: usize,
            v: usize,
        ) -> usize
        where
            Q: RangeMinimumQuery<T = (usize, usize)>,
        {
            let mut left = self.first_pos[u];

            let mut right = self.first_pos[v];

            if left > right {
                std::mem::swap(&mut left, &mut right);
            }

            self.rmq.range_minimum(left, right + 1).1
        }
    }

    use crate::{
        algebraic_structure_impl::*,
        group_theory_id::Min,
        segment_tree_with_static_monoid::Segtree,
    };

    type CommonG = GroupApprox<(usize, usize), Min>;

    #[allow(dead_code)]

    type ETRMQSeg = EulerTourRMQ<Segtree<CommonG>>;

    use crate::sparse_table_with_static_idempotent_semigroup::SparseTable;

    #[allow(dead_code)]

    type ETRMQSpT = EulerTourRMQ<SparseTable<CommonG>>;

    use crate::sqrt_decomposition::SqrtDecomposition;

    #[allow(dead_code)]

    type ETRMQSqD = EulerTourRMQ<SqrtDecomposition<CommonG>>;

    use crate::sparse_table_with_static_idempotent_semigroup::*;

    #[allow(dead_code)]

    type ETRMQDSpT = EulerTourRMQ<DisjointSparseTable<CommonG>>;
}

// TODO:
pub mod dyn_tree {

    //! LCA for dynamic tree.

    // draft
    // by using e.g. lazy segment tree, we can remove a node from tree, and
    // answer for the lca query.
    // if a node is removed, all the descendants' depth are reduced by 1
    // -> range add range minimum query.
    // also, by using dynamic binary search tree like (lazy?) splay tree,
    // we might be able to add a node in addition to removing a node.
}

// TODO:
pub mod dag {

    //! LCA on a DAG.
}
