//! find a farthest node from any node in O(1)
//! a farthest node from each node is one of the tree diameter terminals.

use crate::{
    tree_bfs_parent_depth::bfs,
    tree_diameter_path_unweighted::diameter_path,
};

pub struct FarthestNode {
    u: usize,
    v: usize,
    dep_u: Vec<usize>,
    dep_v: Vec<usize>,
}

// (u, v) denotes tree diameter terminals.
impl FarthestNode {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let path = diameter_path(&g);

        let u = path[0];

        let v = *path.last().unwrap();

        Self { u, v, dep_u: bfs(&g, u).1, dep_v: bfs(&g, v).1 }
    }

    pub fn get(
        &self,
        u: usize,
    ) -> (usize, usize) {
        let d0 = self.dep_u[u];

        let d1 = self.dep_v[u];

        if d0 >= d1 {
            (self.u, d0)
        } else {
            (self.v, d1)
        }
    }
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
            vec![(0, 5, 4), (1, 5, 3), (4, 0, 3)],
        )];

        for (g, q) in cases {
            let f = FarthestNode::new(&g);

            for (u, v, d) in q {
                assert_eq!(f.get(u), (v, d));
            }
        }
    }
}
