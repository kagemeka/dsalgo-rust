use crate::{
    tree_bfs_parent::*,
    tree_restore_path_from_parents::restore_tree_path,
};

pub fn tree_path(
    g: &[Vec<usize>],
    u: usize,
    v: usize,
) -> Vec<usize> {
    let p = tree_bfs_parent(&g, u);

    restore_tree_path(&p, v)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![vec![1], vec![0, 2, 3], vec![1], vec![1, 4], vec![3]];

        assert_eq!(tree_path(&g, 4, 2), [4, 3, 1, 2]);
    }
}
