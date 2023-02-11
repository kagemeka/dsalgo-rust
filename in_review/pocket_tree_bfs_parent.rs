pub fn tree_bfs_parent(
    adjacency_list: Vec<Vec<usize>>,
    root: usize,
) -> Vec<usize> {
    let g = adjacency_list;

    let n = g.len();

    let mut p = vec![n; n];

    p[root] = root;

    let mut que = std::collections::VecDeque::new();

    que.push_back(root);

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if v == p[u] {
                continue;
            }

            p[v] = u;

            que.push_back(v);
        }
    }

    p
}
