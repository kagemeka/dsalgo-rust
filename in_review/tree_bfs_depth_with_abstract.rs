use crate::tree_bfs_abstract::tree_bfs;

pub fn tree_depths(
    tree_edges: &[(usize, usize)],
    root: usize,
) -> Vec<usize> {
    tree_bfs::<usize, _>(
        tree_edges,
        root,
        vec![0; tree_edges.len() + 1],
        |depth, u, v| {
            depth[v] = depth[u] + 1;
        },
    )
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
