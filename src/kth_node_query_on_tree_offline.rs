//! find a node step by k edges from u for each query in offline

use crate::{
    level_ancestor_offline_with_dfs_recurse::level_ancestor,
    tree_diameter_terminal_nodes::diameter_ends,
};

pub fn kth_nodes(
    g: &[Vec<usize>],
    queries: &[(usize, usize)],
) -> Vec<Option<usize>> {
    let (u, v) = diameter_ends(g);

    let res0 = level_ancestor(g, u, queries);

    let res1 = level_ancestor(g, v, queries);

    res0.into_iter()
        .zip(res1.into_iter())
        .map(|(u, v)| {
            if u.is_some() {
                u
            } else if v.is_some() {
                v
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            5,
            vec![(1, 2), (2, 3), (3, 4), (3, 5)],
            vec![(2, 2), (5, 3), (3, 3)],
            vec![4, 1, -1],
        )];

        use crate::{
            adjacency_list_graph_from_edges::graph_from_edges,
            edges_to_0_indexed::edges_to_0_indexed,
        };

        for (n, edges, mut queries, ans) in cases {
            let g = graph_from_edges(n, &edges_to_0_indexed(edges), false);

            for q in queries.iter_mut() {
                q.0 = q.0 - 1;
            }

            let res = kth_nodes(&g, &queries);

            for i in 0..res.len() {
                let x =
                    if ans[i] == -1 { None } else { Some(ans[i] as usize - 1) };

                assert_eq!(res[i], x);
            }
        }
    }
}
