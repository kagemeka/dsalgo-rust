pub struct EulerTour {
    pub tour_edges: Vec<isize>,
    pub parent: Vec<usize>,
    pub depth: Vec<usize>,
    pub edge_in_idx: Vec<usize>,
    pub edge_out_idx: Vec<usize>,
}

impl EulerTour {
    pub fn new(
        g: &[Vec<usize>],
        root: usize,
    ) -> Self {
        let n = g.len();

        let mut tour = Vec::with_capacity(n << 1);

        let mut parent = vec![n; n];

        parent[root] = root;

        let mut depth = vec![0; n];

        let mut in_idx = vec![0; n];

        let mut out_idx = vec![0; n];

        let mut st = vec![root as isize];

        while let Some(u) = st.pop() {
            if u < 0 {
                out_idx[!u as usize] = tour.len();

                tour.push(u);

                continue;
            }

            st.push(!u);

            in_idx[u as usize] = tour.len();

            tour.push(u);

            let u = u as usize;

            for &v in g[u].iter() {
                if v == parent[u] {
                    continue;
                }

                parent[v] = u;

                depth[v] = depth[u] + 1;

                st.push(v as isize);
            }
        }

        Self {
            tour_edges: tour,
            parent,
            depth,
            edge_in_idx: in_idx,
            edge_out_idx: out_idx,
        }
    }

    pub fn tour_nodes(&self) -> Vec<usize> {
        let n = self.tour_edges.len() - 1;

        self.tour_edges[..n]
            .iter()
            .map(
                |&u| {
                    if u >= 0 {
                        u as usize
                    } else {
                        self.parent[!u as usize]
                    }
                },
            )
            .collect()
    }

    pub fn node_first_idx(&self) -> Vec<usize> { self.edge_in_idx.clone() }

    pub fn node_last_idx(&self) -> Vec<usize> {
        let n = self.edge_in_idx.len();

        let mut last_idx = vec![0; n];

        for (i, (&j, &k)) in
            self.edge_in_idx.iter().zip(self.edge_out_idx.iter()).enumerate()
        {
            if j + 1 == k {
                last_idx[i] = i;

                continue;
            }

            let p = self.parent[i];

            if p == p {
                continue;
            }

            last_idx[p] = last_idx[p].max(k);
        }

        last_idx
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![
            vec![1, 2],
            vec![0, 3, 4, 5],
            vec![0],
            vec![1],
            vec![1, 6],
            vec![1],
            vec![4],
        ];

        let et = EulerTour::new(&g, 0);

        assert_eq!(
            et.tour_edges,
            [0, 2, -3, 1, 5, -6, 4, 6, -7, -5, 3, -4, -2, -1,]
        );

        assert_eq!(et.tour_nodes(), [0, 2, 0, 1, 5, 1, 4, 6, 4, 1, 3, 1, 0]);
    }
}
