use crate::strongly_connected_components_path_based::scc;

pub struct TwoSat {
    g: Vec<Vec<usize>>,
    solution: Option<Vec<bool>>,
    updated: bool,
}

impl TwoSat {
    pub fn new(size: usize) -> Self {
        Self { g: vec![vec![]; size << 1], solution: None, updated: false }
    }

    fn n(&self) -> usize { self.g.len() >> 1 }

    /// add a clause (i-th node is f) or (j-th node is g)

    pub fn add_clause(
        &mut self,
        i: usize,
        f: bool,
        j: usize,
        g: bool,
    ) {
        let f = f as usize;

        let g = g as usize;

        self.g[i << 1 | f].push(j << 1 | (g ^ 1));

        self.g[j << 1 | g].push(i << 1 | (f ^ 1));

        // false: 2k
        // true: 2k + 1
        self.updated = false;
    }

    fn update(&mut self) {
        if self.updated {
            return;
        }

        self.updated = true;

        let id = scc(&self.g);

        let n = self.n();

        let mut res = vec![false; n];

        for i in 0..self.n() {
            let u = id[i << 1];

            let v = id[i << 1 | 1];

            if u == v {
                self.solution = None;

                return;
            }

            res[i] = u > v;
        }

        self.solution = Some(res)
    }

    pub fn is_satisfiable(&mut self) -> bool {
        self.update();

        self.solution.is_some()
    }

    /// one of the solutions

    pub fn solve(&mut self) -> Option<Vec<bool>> {
        self.update();

        self.solution.clone()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atcoder_practice2_h() {
        let cases = vec![
            ((2, vec![(1i64, 4), (2, 5), (0, 6)]), Some(vec![4, 2, 6])),
            ((3, vec![(1, 4), (2, 5), (0, 6)]), None),
        ];

        for ((d, a), ans) in cases {
            let n = a.len();

            let mut ts = TwoSat::new(n);

            for i in 0..n {
                for j in 0..i {
                    if (a[i].0 - a[j].0).abs() < d {
                        ts.add_clause(i, false, j, false);
                    }

                    if (a[i].0 - a[j].1).abs() < d {
                        ts.add_clause(i, false, j, true);
                    }

                    if (a[i].1 - a[j].0).abs() < d {
                        ts.add_clause(i, true, j, false);
                    }

                    if (a[i].1 - a[j].1).abs() < d {
                        ts.add_clause(i, true, j, true);
                    }
                }
            }

            let res = ts.solve();

            if res.is_none() {
                assert_eq!(ans, None);

                continue;
            }

            assert!(ans.is_some());

            let ans = ans.unwrap();

            let res = res.unwrap();

            for i in 0..n {
                assert_eq!(if res[i] { a[i].0 } else { a[i].1 }, ans[i]);
            }
        }
    }
}
