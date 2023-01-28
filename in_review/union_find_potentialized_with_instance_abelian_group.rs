pub trait AbelianGroup {
    type T;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;

    fn inv(
        &self,
        x: Self::T,
    ) -> Self::T;
}

pub struct PotentialUnionFind<G: AbelianGroup> {
    g: G,
    a: Vec<isize>, // parent, or neg-size if root
    rh: Vec<G::T>, // relative potential from parent, or identity if root.
}

impl<G: AbelianGroup> PotentialUnionFind<G>
where
    G::T: Clone,
{
    pub fn size(&self) -> usize { self.a.len() }

    pub fn new(
        g: G,
        size: usize,
    ) -> Self {
        let rh = vec![g.e(); size];

        Self { g, a: vec![-1; size], rh }
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

        self.rh[u] = self.g.op(self.rh[u].clone(), self.rh[p].clone());

        self.a[u] as usize
    }

    // potential from the root node.
    fn h(
        &mut self,
        u: usize,
    ) -> G::T {
        self.root(u);

        self.rh[u].clone()
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

    /// relative potential from u to v

    pub fn diff(
        &mut self,
        u: usize,
        v: usize,
    ) -> Result<G::T, &'static str> {
        if !self.same(u, v) {
            Err("belongs to different components")
        } else {
            let hu = self.h(u);

            let hv = self.h(v);

            Ok(self.g.op(self.g.inv(hu), hv))
        }
    }

    pub fn unite(
        &mut self,
        mut u: usize,
        mut v: usize,
        diff: G::T, // potential from u to v
    ) where
        G::T: PartialEq,
    {
        let hu = self.h(u);

        let hv = self.h(v);

        let mut d = self.g.op(self.g.op(hu, diff), self.g.inv(hv));

        u = self.root(u);

        v = self.root(v);

        if u == v {
            debug_assert!(d == self.g.e());

            return;
        }

        if self.a[u] > self.a[v] {
            std::mem::swap(&mut u, &mut v);

            d = self.g.inv(d);
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
        struct G;

        impl AbelianGroup for G {
            type T = i32;

            fn e(&self) -> Self::T { 0 }

            fn inv(
                &self,
                x: Self::T,
            ) -> Self::T {
                -x
            }

            fn op(
                &self,
                l: Self::T,
                r: Self::T,
            ) -> Self::T {
                l + r
            }
        }

        let mut uf = PotentialUnionFind::new(G {}, 6);

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
