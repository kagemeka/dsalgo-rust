use crate::fenwick_tree_with_instance_commutative_monoid_1_indexed::*;

pub struct DualFenwick<G: Monoid>(pub(crate) Fenwick<G>);

impl<G: Monoid> DualFenwick<G>
where
    G::T: Clone,
{
    pub fn new(
        g: G,
        size: usize,
    ) -> Self {
        Self(Fenwick::new(g, size))
    }

    pub fn size(&self) -> usize { self.0.size() }

    pub fn operate_ge(
        &mut self,
        i: usize,
        x: G::T,
    ) {
        self.0.operate(i, x)
    }

    pub fn get(
        &self,
        i: usize,
    ) -> G::T {
        self.0.fold_lt(i + 1)
    }

    /// \forall{j=0..=i - 1} not is_ok(a_j),
    /// \forall{j=i..n} is_ok(a_i).

    pub fn binary_search<F>(
        &self,
        is_ok: F,
    ) -> usize
    where
        F: Fn(&G::T) -> bool,
    {
        self.0.max_right(&|v: &G::T| !is_ok(v))
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        #[derive(Debug, Clone)]

        struct G;

        impl Monoid for G {
            type T = i32;

            fn e(&self) -> Self::T { 0 }

            fn op(
                &self,
                l: Self::T,
                r: Self::T,
            ) -> Self::T {
                l + r
            }
        }

        let n = 5;

        let mut fw = DualFenwick::new(G {}, n);

        assert_eq!(fw.size(), n);

        fw.operate_ge(1, 1);

        for i in 0..n {
            assert_eq!(fw.get(i), if i == 0 { 0 } else { 1 });
        }

        assert_eq!(fw.binary_search(|v| v >= &0), 0);

        assert_eq!(fw.binary_search(|v| v >= &1), 1);

        assert_eq!(fw.binary_search(|v| v >= &2), n);
    }
}
