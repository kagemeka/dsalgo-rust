use crate::{
    fenwick_tree_dual_with_instance_commutative_monoid::*,
    fenwick_tree_with_instance_abelian_group_1_indexed::*,
    fenwick_tree_with_instance_commutative_monoid_1_indexed::*,
};

impl<G: AbelianGroup> From<(G, &[G::T])> for DualFenwick<G>
where
    G::T: Clone,
{
    fn from(p: (G, &[G::T])) -> Self {
        let (g, data) = p;

        let mut data = data.to_vec();

        for i in (1..data.len()).rev() {
            data[i] = g.op(g.inv(data[i - 1].clone()), data[i].clone());
        }

        Self(Fenwick::from((g, data.as_slice())))
    }
}

impl<G: AbelianGroup> DualFenwick<G>
where
    G::T: Clone,
{
    pub fn operate(
        &mut self,
        l: usize,
        r: usize,
        x: G::T,
    ) {
        assert!(l < r && r <= self.size());

        if r < self.size() {
            self.operate_ge(r, self.0.g.inv(x.clone()));
        }

        self.operate_ge(l, x);
    }

    pub fn binary_search_from<F>(
        &self,
        is_ok: F,
        l: usize,
    ) -> usize
    where
        F: Fn(&G::T) -> bool,
    {
        assert!(l <= self.size());

        let prod_lt = if l == 0 { self.0.g.e() } else { self.get(l - 1) };

        self.0.max_right_from(
            &|prod_ge: &G::T| {
                !is_ok(&self.0.g.op(prod_lt.clone(), prod_ge.clone()))
            },
            l,
        )
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

        impl AbelianGroup for G {
            fn inv(
                &self,
                x: Self::T,
            ) -> Self::T {
                -x
            }
        }

        let a = vec![0, 1, 0, 0];

        let n = a.len();

        let mut fw = DualFenwick::from((G {}, a.as_slice()));

        for i in 0..n {
            assert_eq!(fw.get(i), a[i]);
        }

        fw.operate(2, 3, 1);

        for i in 0..n {
            assert_eq!(fw.get(i), a[i] + if i == 2 { 1 } else { 0 });
        }

        assert_eq!(fw.binary_search_from(|v| v <= &0, 1), 3);

        assert_eq!(fw.binary_search_from(|v| v <= &-1, 1), n);
    }
}
