pub struct UnionFind(Vec<isize>);

impl UnionFind {
    pub fn new(size: usize) -> Self { Self(vec![-1; size]) }

    pub fn size(&self) -> usize { self.0.len() }

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

        if self.0[u] > self.0[v] {
            std::mem::swap(&mut u, &mut v);
            // (u, v) = (v, u);
        }

        self.0[u] += self.0[v];

        self.0[v] = u as isize;
    }

    pub fn size_of(
        &mut self,
        mut u: usize,
    ) -> usize {
        u = self.root(u);

        -self.0[u] as usize
    }

    pub fn same(
        &mut self,
        u: usize,
        v: usize,
    ) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn labels(&mut self) -> Vec<usize> {
        let n = self.size();

        let mut labels = vec![n; n];

        let mut l = 0;

        for i in 0..n {
            let r = self.root(i);

            if labels[r] == n {
                labels[r] = l;

                l += 1;
            }

            labels[i] = labels[r];
        }

        labels
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut uf = UnionFind::new(10);

        assert_eq!(uf.size_of(0), 1);

        uf.unite(3, 9);

        assert_eq!(uf.size_of(3), 2);
    }

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
                    assert_eq!(if uf.same(u, v) { 1 } else { 0 }, ans);
                }
            }
        }
    }
}
