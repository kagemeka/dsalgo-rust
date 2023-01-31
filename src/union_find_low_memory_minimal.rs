pub struct UnionFind(Vec<isize>);

impl UnionFind {
    pub fn new(size: usize) -> Self { Self(vec![-1; size]) }

    pub fn root(
        &mut self,
        u: usize,
    ) -> usize {
        if self.0[u] < 0 {
            return u;
        }

        let r = self.root(self.0[u] as usize);

        self.0[u] = r as isize;

        return r;
    }

    pub fn unite(
        &mut self,
        mut u: usize,
        mut v: usize,
    ) {
        u = self.root(u);

        v = self.root(v);

        if u == v {
            return;
        }

        use std::mem::swap;

        if self.0[u] > self.0[v] {
            swap(&mut u, &mut v);
        }

        self.0[u] += self.0[v];

        self.0[v] = u as isize;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_practice2_b() {
        let cases = vec![(
            4,
            vec![
                ((1, 0, 1), 0),
                ((0, 0, 1), 2),
                ((0, 2, 3), 2),
                ((1, 0, 1), 1),
                ((1, 1, 2), 0),
                ((0, 0, 2), 2),
                ((1, 1, 3), 1),
            ],
        )];

        for (n, q) in cases {
            let mut uf = UnionFind::new(n);

            for ((t, u, v), ans) in q {
                if t == 0 {
                    uf.unite(u, v);
                } else {
                    assert_eq!(
                        if uf.root(u) == uf.root(v) { 1 } else { 0 },
                        ans
                    );
                }
            }
        }
    }
}
