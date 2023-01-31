pub struct PotentialUnionFind {
    a: Vec<isize>,
    rh: Vec<i64>,
}

impl PotentialUnionFind {
    pub fn size(&self) -> usize { self.a.len() }

    pub fn new(size: usize) -> Self {
        Self { a: vec![-1; size], rh: vec![0; size] }
    }

    pub fn root(
        &mut self,
        u: usize,
    ) -> usize {
        if self.a[u] < 0 {
            return u;
        }

        let p = self.a[u] as usize;

        self.a[u] = self.root(p) as isize;

        self.rh[u] += self.rh[p];

        self.a[u] as usize
    }

    fn h(
        &mut self,
        u: usize,
    ) -> i64 {
        self.root(u);

        self.rh[u]
    }

    pub fn size_of(
        &mut self,
        u: usize,
    ) -> usize {
        let u = self.root(u);

        -self.a[u] as usize
    }

    pub fn same(
        &mut self,
        u: usize,
        v: usize,
    ) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn diff(
        &mut self,
        u: usize,
        v: usize,
    ) -> Result<i64, &'static str> {
        if !self.same(u, v) {
            Err("belongs to different components")
        } else {
            Ok(self.h(v) - self.h(u))
        }
    }

    pub fn unite(
        &mut self,
        mut u: usize,
        mut v: usize,
        diff: i64,
    ) {
        let mut d = self.h(u) + diff - self.h(v);

        u = self.root(u);

        v = self.root(v);

        if u == v {
            debug_assert!(d == 0);

            return;
        }

        if self.a[u] > self.a[v] {
            std::mem::swap(&mut u, &mut v);

            d = -d;
        }

        self.a[u] += self.a[v];

        self.a[v] = u as isize;

        self.rh[v] = d;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_potential_uf() {
        let mut uf = PotentialUnionFind::new(6);

        assert_eq!(uf.size_of(0), 1);

        assert!(uf.diff(0, 5).is_err());

        uf.unite(0, 1, 1);

        uf.unite(5, 4, 2);

        uf.unite(1, 2, 3);

        uf.unite(4, 3, 4);

        uf.unite(2, 3, 5);

        assert_eq!(uf.size_of(4), 6);

        assert_eq!(uf.diff(0, 5), Ok(3));
    }
}
