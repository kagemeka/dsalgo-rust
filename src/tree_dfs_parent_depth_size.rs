pub fn tree_dfs(
    g: &[Vec<usize>],
    root: usize,
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = g.len();

    let mut size = vec![1; n];

    let mut parent = vec![n; n];

    let mut depth = vec![0; n];

    fn dfs(
        g: &[Vec<usize>],
        parent: &mut [usize],
        depth: &mut [usize],
        size: &mut [usize],
        u: usize,
    ) {
        let d = depth[u] + 1;

        for &v in g[u].iter() {
            if v == parent[u] {
                continue;
            }

            parent[v] = u;

            depth[v] = d;

            dfs(g, parent, depth, size, v);

            size[u] += size[v];
        }
    }

    parent[root] = root;

    dfs(g, &mut parent, &mut depth, &mut size, root);

    (parent, depth, size)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (5, vec![(0, 1), (0, 2), (1, 4), (3, 4)]),
            (vec![0, 0, 0, 4, 1], vec![0, 1, 1, 3, 2], vec![5, 3, 1, 1, 2]),
        )];

        use crate::adjacency_list_graph_from_edges::graph_from_edges;

        for ((n, edges), ans) in cases {
            let g = graph_from_edges(n, &edges, false);

            assert_eq!(tree_dfs(&g, 0), ans);
        }
    }
}
