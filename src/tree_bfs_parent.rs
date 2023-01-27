/// parent of root := root

pub fn tree_bfs_parent(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<usize> {
    let n = g.len();

    let mut parent = vec![n; n];

    parent[root] = root;

    let mut que = std::collections::VecDeque::new();

    que.push_back(root);

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if v == parent[u] {
                continue;
            }

            parent[v] = u;

            que.push_back(v);
        }
    }

    parent
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
