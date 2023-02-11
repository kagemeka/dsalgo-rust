use crate::lower_bound_on_slice::lower_bound;

pub struct GetNodesInRectangle {
    x_list: Vec<i64>,
    y_list: Vec<Vec<(i64, usize)>>,
}

impl GetNodesInRectangle {
    pub fn new(xy: &[(i64, i64)]) -> Self {
        let mut x_list: Vec<_> = xy.iter().map(|&(x, _)| x).collect();

        x_list.sort();

        x_list.dedup();

        let m = x_list.len();

        let mut y_list = vec![vec![]; m];

        for (i, &(x, y)) in xy.iter().enumerate() {
            y_list[lower_bound(&x_list, &x)].push((y, i));
        }

        for y in y_list.iter_mut() {
            y.sort();
        }

        Self { x_list, y_list }
    }

    /// x0 <= x < x1, y0 <= y < y1

    pub fn enumerate_nodes(
        &self,
        x0: i64,
        y0: i64,
        x1: i64,
        y1: i64,
    ) -> Vec<usize> {
        let i0 = lower_bound(&self.x_list, &x0);

        let i1 = lower_bound(&self.x_list, &x1);

        let mut res = vec![];

        for y_list in self.y_list[i0..i1].iter() {
            let i0 = lower_bound(y_list, &(y0, 0));

            let i1 = lower_bound(y_list, &(y1, 0));

            for &(_, id) in y_list[i0..i1].iter() {
                res.push(id);
            }
        }

        res
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
