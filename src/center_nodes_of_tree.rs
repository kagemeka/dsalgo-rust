//! find a pair of center nodes of the given unweited tree graph.
//! if tree diameter is even
//! or number of nodes in the path is odd,
//! two center nodes are same.

use crate::tree_diameter_path_unweighted::diameter_path;

pub fn center_of_nodes(g: &[Vec<usize>]) -> (usize, usize) {
    let path = diameter_path(g);

    let n = path.len();

    (path[n >> 1], path[(n - 1) >> 1])
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
            (2, 2),
        )];

        for (g, ans) in cases {
            assert_eq!(center_of_nodes(&g), ans);
        }
    }
}
