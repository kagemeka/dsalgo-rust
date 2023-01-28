pub fn tree_dfs_size(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<usize> {
    let n = g.len();

    let mut size = vec![1; n];

    fn dfs(
        g: &[Vec<usize>],
        size: &mut [usize],
        u: usize,
        p: usize,
    ) {
        for &v in g[u].iter() {
            if v == p {
                continue;
            }

            dfs(g, size, v, u);

            size[u] += size[v];
        }
    }

    dfs(g, &mut size, root, n);

    size
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (5, vec![(0, 1), (0, 2), (1, 4), (3, 4)]),
            vec![5, 3, 1, 1, 2],
        )];

        use crate::adjacency_list_graph_from_edges::graph_from_edges;

        for ((n, edges), ans) in cases {
            let g = graph_from_edges(n, &edges, false);

            assert_eq!(tree_dfs_size(&g, 0), ans);
        }
    }
}
