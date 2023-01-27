use crate::{
    argmax::argmax,
    tree_bfs_parent_depth::bfs,
};

/// return (node, dist)

pub fn farthest_node(
    g: &[Vec<usize>],
    u: usize,
) -> (usize, usize) {
    let (_, d) = bfs(g, u);

    let v = argmax(&d);

    (v, d[v])
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
            0,
            (5, 4),
        )];

        for (g, u, ans) in cases {
            assert_eq!(farthest_node(&g, u), ans);
        }
    }
}
