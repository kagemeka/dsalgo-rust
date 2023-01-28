//! return undirected edges
//! input: bidirected adjacency list (undirected graph)
//! output: undirected edges [(u, v)] u < v. (but not sorted vertically)
//! (deal with no given priority between u and v, and among edges.)

pub fn bfs_tree(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<(usize, usize)> {
    let n = g.len();

    let mut used = Vec::with_capacity(n - 1);

    let mut que = std::collections::VecDeque::new();

    que.push_back(root);

    let mut to_que = vec![false; n];

    to_que[root] = true;

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if to_que[v] {
                continue;
            }

            que.push_back(v);

            to_que[v] = true;

            used.push(if u < v { (u, v) } else { (v, u) });
        }
    }

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
            vec![(0, 1), (0, 3), (1, 2)],
        )];

        for (g, root, ans) in cases {
            let edges = bfs_tree(&g, root);

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
            vec![(0, 4), (0, 3), (0, 1), (0, 5), (2, 4)],
        )];

        use crate::{
            adjacency_list_graph_from_edges::graph_from_edges,
            edges_to_0_indexed::edges_to_0_indexed,
        };

        for ((n, mut edges, root), ans) in cases {
            edges = edges_to_0_indexed(edges);

            let g = graph_from_edges(n, &edges, false);

            assert_eq!(bfs_tree(&g, root), ans);
        }
    }
}
