use crate::lower_bound_on_slice::lower_bound;

pub struct GetNodesInRectangle {
    nodes: Vec<(i64, i64, usize)>,
}

impl GetNodesInRectangle {
    pub fn new(xy: &[(i64, i64)]) -> Self {
        let mut nodes: Vec<_> =
            xy.iter().enumerate().map(|(i, &(x, y))| (x, y, i)).collect();

        nodes.sort();

        Self { nodes }
    }

    /// x0 <= x < x1, y0 <= y < y1

    pub fn enumerate_nodes(
        &self,
        x0: i64,
        y0: i64,
        x1: i64,
        y1: i64,
    ) -> Vec<usize> {
        let i0 = lower_bound(&self.nodes, &(x0, y0, 0));

        let i1 = lower_bound(&self.nodes, &(x1 - 1, y1, 0));

        self.nodes[i0..i1]
            .iter()
            .filter_map(
                |&(_, y, i)| {
                    if y0 <= y && y < y1 {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let xy = vec![(2, 1), (2, 2), (4, 2), (6, 2), (3, 3), (5, 4)];

        let queries = vec![
            ((2, 4, 0, 4), vec![0, 1, 4, 2]),
            ((4, 10, 2, 5), vec![2, 5, 3]),
        ];

        let f = GetNodesInRectangle::new(&xy);

        for ((x0, x1, y0, y1), ans) in queries {
            assert_eq!(f.enumerate_nodes(x0, y0, x1 + 1, y1 + 1), ans);
        }
    }
}
