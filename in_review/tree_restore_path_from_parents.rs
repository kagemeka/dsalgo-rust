/// it should be satisfied that parent[root] = root

pub fn restore_tree_path(
    parent: &[usize],
    mut v: usize,
) -> Vec<usize> {
    let mut path = Vec::new();

    loop {
        path.push(v);

        let p = parent[v];

        if p == v {
            break;
        }

        v = p;
    }

    path.reverse();

    path
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (
                vec![
                    vec![1],
                    vec![0, 2],
                    vec![1, 3, 4],
                    vec![2],
                    vec![2, 5],
                    vec![4],
                ],
                0,
            ),
            (5, vec![0, 1, 2, 4, 5]),
        )];

        use crate::tree_bfs_parent::tree_bfs_parent;

        for ((g, root), (v, ans)) in cases {
            let p = tree_bfs_parent(&g, root);

            assert_eq!(restore_tree_path(&p, v), ans);
        }
    }
}
