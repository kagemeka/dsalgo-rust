use crate::adjacency_list_graph_with_edge_id_from_edges::*;

pub fn dfs_tree(
    n: usize,
    edges: &[(usize, usize)],
    root: usize,
) -> Vec<bool> {
    let m = edges.len();

    let mut used = vec![false; m];

    let mut visited = vec![false; m];

    fn dfs(
        g: &[Vec<(usize, usize)>],
        visited: &mut [bool],
        used: &mut [bool],
        u: usize,
    ) {
        visited[u] = true;

        for &(v, eid) in g[u].iter() {
            if visited[v] {
                continue;
            }

            used[eid] = true;

            dfs(g, visited, used, v);
        }
    }

    let g = graph_from_edges(n, edges, false);

    dfs(&g, &mut visited, &mut used, root);

    used
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (
                (4, vec![(0, 1), (0, 3), (1, 2), (1, 3), (2, 3)]),
                vec![true, false, true, false, true],
            ),
            (
                (
                    6,
                    vec![
                        (0, 1),
                        (0, 3),
                        (0, 4),
                        (0, 5),
                        (1, 3),
                        (1, 5),
                        (2, 3),
                        (2, 4),
                    ],
                ),
                vec![true, false, false, false, true, true, true, true],
            ),
        ];

        for ((n, edges), ans) in cases {
            assert_eq!(dfs_tree(n, &edges, 0), ans);
        }
    }
}
