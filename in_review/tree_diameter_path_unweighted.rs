use crate::{
    argmax::argmax,
    tree_bfs_parent_depth::bfs,
};

pub fn diameter_path(g: &[Vec<usize>]) -> Vec<usize> {
    let (_, dep) = bfs(&g, 0);

    let (parent, dep) = bfs(&g, argmax(&dep));

    let mut v = argmax(&dep);

    let mut path = vec![v];

    for _ in 0..dep[v] {
        v = parent[v];

        path.push(v);
    }

    path
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            vec![
                vec![1],
                vec![0, 2],
                vec![1, 3, 4],
                vec![2],
                vec![2, 5],
                vec![4],
            ],
            vec![0, 1, 2, 4, 5],
        )];

        for (g, ans) in cases {
            assert_eq!(diameter_path(&g), ans);
        }
    }
}
