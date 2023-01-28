pub fn dfs_tree(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<(usize, usize)> {
    let n = g.len();

    let mut visited = vec![false; n];

    let mut used = Vec::with_capacity(n - 1);

    fn dfs(
        g: &[Vec<usize>],
        visited: &mut [bool],
        used: &mut Vec<(usize, usize)>,
        u: usize,
    ) {
        visited[u] = true;

        for &v in g[u].iter() {
            if visited[v] {
                continue;
            }

            used.push(if u < v { (u, v) } else { (v, u) });

            dfs(g, visited, used, v);
        }
    }

    dfs(g, &mut visited, &mut used, root);

    used
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            vec![vec![1, 3], vec![0, 2, 3], vec![1, 3], vec![0, 1, 2]],
            0,
            vec![(0, 1), (1, 2), (2, 3)],
        )];

        for (g, root, ans) in cases {
            let edges = dfs_tree(&g, root);

            assert_eq!(edges, ans);
        }
    }

    #[test]

    fn test_atcoder_abc251_f() {
        let cases = vec![(
            (
                6,
                vec![
                    (5, 1),
                    (4, 3),
                    (1, 4),
                    (3, 5),
                    (1, 2),
                    (2, 6),
                    (1, 6),
                    (4, 2),
                ],
                0,
            ),
            vec![(0, 4), (2, 4), (2, 3), (1, 3), (1, 5)],
        )];

        use crate::{
            adjacency_list_graph_from_edges::graph_from_edges,
            edges_to_0_indexed::edges_to_0_indexed,
        };

        for ((n, mut edges, root), ans) in cases {
            edges = edges_to_0_indexed(edges);

            let g = graph_from_edges(n, &edges, false);

            assert_eq!(dfs_tree(&g, root), ans);
        }
    }
}
