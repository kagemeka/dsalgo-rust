use crate::strongly_connected_components_topological_sort::toposort;

pub struct SCC<'a> {
    g: &'a [Vec<usize>],
    preorder: Vec<usize>,
    low: Vec<usize>,
    order: Vec<usize>,
    ord: usize,
    labels: Vec<usize>,
    label: usize,
}

impl<'a> SCC<'a> {
    pub fn calc(g: &'a [Vec<usize>]) -> Vec<usize> {
        let n = g.len();

        let mut scc = Self::new(g);

        for i in 0..n {
            if scc.labels[i] == n {
                scc.labeling(i);
            }
        }

        toposort(scc.labels)
    }

    fn new(g: &'a [Vec<usize>]) -> Self {
        let n = g.len();

        Self {
            g,
            preorder: Vec::with_capacity(n),
            low: Vec::with_capacity(n),
            order: vec![n; n],
            ord: 0,
            labels: vec![n; n],
            label: 0,
        }
    }

    fn labeling(
        &mut self,
        u: usize,
    ) {
        self.order[u] = self.ord;

        self.ord += 1;

        self.preorder.push(u);

        self.low.push(u);

        let n = self.g.len();

        for &v in self.g[u].iter() {
            if self.order[v] == n {
                self.labeling(v);
            } else if self.labels[v] == n {
                while self.order[*self.low.last().unwrap()] > self.order[v] {
                    self.low.pop();
                }
            }
        }

        if *self.low.last().unwrap() != u {
            return;
        }

        loop {
            let v = self.preorder.pop().unwrap();

            self.labels[v] = self.label;

            if v == u {
                break;
            }
        }

        self.label += 1;

        self.low.pop();
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (6, vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)]),
            vec![3, 1, 2, 3, 1, 0],
        )];

        for ((n, edges), ans) in cases {
            let mut g = vec![vec![]; n];

            for (u, v) in edges {
                g[u].push(v);
            }

            assert_eq!(SCC::calc(&g), ans);
        }
    }
}
