//! make a bfs tree from given undirected csgraph edges.
//! return added flagged boolean result.
//! if the given graph is not connected,
//! make a bfs tree for the connected component containing source node.

use crate::adjacency_list_graph_with_edge_id_from_edges::*;

pub fn bfs_tree(
    n: usize,
    edges: &[(usize, usize)],
    root: usize,
) -> Vec<bool> {
    let g = graph_from_edges(n, edges, false);

    let m = edges.len();

    let mut used = vec![false; m];

    let mut que = std::collections::VecDeque::new();

    que.push_back(root);

    let mut to_que = vec![false; n];

    to_que[root] = true;

    while let Some(u) = que.pop_front() {
        for &(v, eid) in g[u].iter() {
            if to_que[v] {
                continue;
            }

            que.push_back(v);

            to_que[v] = true;

            used[eid] = true;
        }
    }

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
                vec![true, true, true, false, false],
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
                vec![true, true, true, true, false, false, true, false],
            ),
        ];

        for ((n, edges), ans) in cases {
            assert_eq!(bfs_tree(n, &edges, 0), ans);
        }
    }
}
