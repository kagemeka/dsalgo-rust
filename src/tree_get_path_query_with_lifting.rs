use crate::tree_bfs_parent_depth::bfs;

pub struct TreePath {
    parent: Vec<usize>,
    depth: Vec<usize>,
}

impl TreePath {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let (parent, depth) = bfs(g, 0);

        Self { parent, depth }
    }

    pub fn get(
        &self,
        mut u: usize,
        mut v: usize,
    ) -> Vec<usize> {
        let mut l = vec![];

        let mut r = vec![];

        while u != v {
            if self.depth[u] >= self.depth[v] {
                l.push(u);

                u = self.parent[u];
            }

            if self.depth[v] > self.depth[u] {
                r.push(v);

                v = self.parent[v];
            }
        }

        l.push(u);

        r.reverse();

        l.append(&mut r);

        l
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![vec![1], vec![0, 2, 3], vec![1], vec![1, 4], vec![3]];

        let f = TreePath::new(&g);

        assert_eq!(f.get(4, 2), [4, 3, 1, 2]);
    }
}
