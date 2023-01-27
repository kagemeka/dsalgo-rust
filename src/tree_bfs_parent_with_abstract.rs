use crate::tree_bfs_abstract::tree_bfs;

pub fn tree_parents(
    e: &[(usize, usize)],
    r: usize,
) -> Vec<Option<usize>> {
    tree_bfs::<Option<usize>, _>(
        e,
        r,
        vec![None; e.len() + 1],
        |parent, u, v| {
            parent[v] = Some(u);
        },
    )
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
