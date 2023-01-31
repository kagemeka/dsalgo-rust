use crate::{
    algebraic_structure::*,
    union_find_traits::*,
};

pub struct PotentialUF<G: AbelianGroup> {
    a: Vec<isize>, // neg-size or parent
    rp: Vec<G::S>, // root: identity, other: relative potential from parent
}

impl<G: AbelianGroup> Size for PotentialUF<G> {
    fn size(&self) -> usize { self.a.len() }
}

/// Potentialized (or Weighted) Union Find

impl<G> PotentialUF<G>
where
    G: AbelianGroup,
    G::S: Clone,
{
    pub fn new(size: usize) -> Self {
        Self { a: vec![-1; size], rp: (0..size).map(|_| G::e()).collect() }
    }

    /// relative potential from the root.

    fn h(
        &mut self,
        u: usize,
    ) -> G::S {
        self.root(u);

        self.rp[u].clone()
    }

    /// potential v against u.

    pub fn diff(
        &mut self,
        u: usize,
        v: usize,
    ) -> Result<G::S, &'static str> {
        if !self.same(u, v) {
            Err("different components")
        } else {
            Ok(G::op(G::inv(self.h(u)), self.h(v)))
        }
    }

    pub fn unite(
        &mut self,
        mut u: usize,
        mut v: usize,
        d: G::S, // potential v against u
    ) where
        G::S: PartialEq,
    {
        let mut d = G::op(G::op(self.h(u), d), G::inv(self.h(v).clone()));

        u = self.root(u);

        v = self.root(v);

        if u == v {
            debug_assert!(d == G::e());

            return;
        }

        if self.a[u] > self.a[v] {
            std::mem::swap(&mut u, &mut v);

            d = G::inv(d);
        }

        self.a[u] += self.a[v];

        self.a[v] = u as isize;

        self.rp[v] = d;
    }
}

impl<G> Root for PotentialUF<G>
where
    G: AbelianGroup,
    G::S: Clone,
{
    fn root(
        &mut self,
        u: usize,
    ) -> usize {
        if self.a[u] < 0 {
            return u;
        }

        let p = self.a[u] as usize;

        self.a[u] = self.root(p) as isize;

        self.rp[u] = G::op(self.rp[u].clone(), self.rp[p].clone());

        self.a[u] as usize
    }
}

impl<G> SizeOf for PotentialUF<G>
where
    G: AbelianGroup,
    G::S: Clone,
{
    fn size_of(
        &mut self,
        u: usize,
    ) -> usize
    where
        G::S: Clone,
    {
        let u = self.root(u);

        -self.a[u] as usize
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_potential_uf() {
        use crate::{
            algebraic_structure_impl::*,
            group_theory_id::Additive,
        };

        let mut uf = PotentialUF::<GroupApprox<i32, Additive>>::new(6);

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
