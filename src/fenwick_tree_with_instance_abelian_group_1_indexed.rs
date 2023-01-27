use crate::fenwick_tree_with_instance_commutative_monoid_1_indexed::*;

pub trait AbelianGroup: Monoid {
    fn inv(
        &self,
        x: Self::T,
    ) -> Self::T;
}

impl<G: AbelianGroup> Fenwick<G>
where
    G::T: Clone,
{
    pub fn fold(
        &self,
        l: usize,
        r: usize,
    ) -> G::T {
        assert!(l <= r);

        self.g.op(self.g.inv(self.fold_lt(l)), self.fold_lt(r))
    }

    pub fn max_right_from<F>(
        &self,
        is_ok: F,
        l: usize,
    ) -> usize
    where
        F: Fn(&G::T) -> bool,
    {
        let n = self.size();

        assert!(l <= n);

        let mut i = 0;

        let mut v = self.g.inv(self.fold_lt(l));

        let mut d = (n + 1).next_power_of_two();

        loop {
            d >>= 1;

            if d == 0 {
                debug_assert!(l <= i && i <= n);

                return i;
            }

            if i + d > n {
                continue;
            }

            let nv = self.g.op(v.clone(), self.node[i + d].clone());

            if i + d <= l || is_ok(&nv) {
                i += d;

                v = nv;
            }
        }
    }

    pub fn min_left_from<F>(
        &self,
        is_ok: F,
        r: usize,
    ) -> usize
    where
        F: Fn(&G::T) -> bool,
    {
        let n = self.size();

        assert!(r <= n);

        if r == 0 {
            return 0;
        }

        let mut v = self.fold_lt(r);

        let mut d = (n + 1).next_power_of_two();

        if is_ok(&v) {
            return 0;
        }

        let mut i = 1;

        loop {
            d >>= 1;

            if d == 0 {
                debug_assert!(i <= r);

                return i;
            }

            if i + d > r {
                continue;
            }

            let nv =
                self.g.op(self.g.inv(self.node[i + d - 1].clone()), v.clone());

            if !is_ok(&nv) {
                i += d;

                v = nv;
            }
        }
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

        let mut fw = Fenwick::from((G {}, a.as_slice()));

        assert_eq!(fw.fold(0, n), 1);

        fw.operate(2, 1);

        assert_eq!(fw.max_right_from(|v| v <= &1, 1), 2);

        assert_eq!(fw.max_right_from(|v| v <= &1, 2), n);

        assert_eq!(fw.min_left_from(|v| v <= &1, n), 2);

        assert_eq!(fw.min_left_from(|v| v <= &2, n), 0);

        assert_eq!(fw.min_left_from(|v| v <= &1, 2), 0);

        assert_eq!(fw.min_left_from(|v| v <= &0, n), 3);
    }
}
