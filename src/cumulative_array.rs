//! static range query for group like.

use crate::algebraic_structure::*;

pub struct CumulativeArray<G: Monoid> {
    s: Vec<G::S>,
}

impl<G: Monoid> CumulativeArray<G>
where
    G::S: Clone,
{
    pub fn new(mut a: Vec<G::S>) -> Self {
        let size = a.len();

        let mut s = vec![G::e()];

        s.append(&mut a);

        for i in 0..size {
            s[i + 1] = G::op(s[i].clone(), s[i + 1].clone());
        }

        Self { s }
    }

    pub fn size(&self) -> usize { self.s.len() - 1 }

    pub fn reduce_lt(
        &self,
        i: usize,
    ) -> G::S {
        self.s[i].clone()
    }

    pub fn reduce(
        &self,
        l: usize,
        r: usize,
    ) -> G::S
    where
        G: Group,
    {
        assert!(l < r && r <= self.size());

        G::op(G::inv(self.reduce_lt(l)), self.reduce_lt(r))
    }
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
