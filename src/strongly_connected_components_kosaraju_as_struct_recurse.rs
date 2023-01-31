pub struct SCC {
    g: Vec<Vec<usize>>,
    post_order: Vec<usize>,
    state: Vec<usize>,
}

impl SCC {
    pub fn calc(g: Vec<Vec<usize>>) -> Vec<usize> {
        let n = g.len();

        let mut scc = SCC::new(g);

        for u in 0..n {
            if scc.state[u] == 0 {
                scc.calc_topological_rev_ord(u);
            }
        }

        scc.transpose();

        let mut label = 0;

        for u in scc.post_order.clone().into_iter().rev() {
            if scc.state[u] != n {
                continue;
            }

            scc.labeling(label, u);

            label += 1;
        }

        scc.state
    }

    fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();

        Self { g, post_order: vec![], state: vec![0; n] }
    }

    fn transpose(&mut self) {
        let n = self.g.len();

        let mut g = vec![vec![]; n];

        for (i, edges) in self.g.iter().enumerate() {
            for &j in edges.iter() {
                g[j].push(i);
            }
        }

        self.g = g;
    }

    fn calc_topological_rev_ord(
        &mut self,
        u: usize,
    ) {
        let n = self.g.len();

        self.state[u] = n;

        for v in self.g[u].to_owned().into_iter() {
            if self.state[v] == 0 {
                self.calc_topological_rev_ord(v);
            }
        }

        self.post_order.push(u);
    }

    fn labeling(
        &mut self,
        label: usize,
        u: usize,
    ) {
        self.state[u] = label;

        for v in self.g[u].clone().into_iter() {
            if self.state[v] == self.g.len() {
                self.labeling(label, v);
            }
        }
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

            assert_eq!(SCC::calc(g), ans);
        }
    }
}
