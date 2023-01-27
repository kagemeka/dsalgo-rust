pub fn tree_bfs_depth(
    adjacency_list: Vec<Vec<usize>>,
    root: usize,
) -> Vec<usize> {
    let g = adjacency_list;

    let n = g.len();

    let mut d = vec![0; n];

    let mut que = std::collections::VecDeque::new();

    que.push_back((root, root));

    while let Some((u, p)) = que.pop_front() {
        for &v in g[u].iter() {
            if v == p {
                continue;
            }

            d[v] = d[u] + 1;

            que.push_back((v, u));
        }
    }

    d
}
