//! fenwick tree (binary indexed tree)

use crate::{
    algebraic_structure::*,
    binary_function::*,
};

/// Node Indices
/// (case $|given array| = 8$,
/// internally 1-indexed implemetation)
/// |8              |
/// |4      |       |
/// |2  |   |6  |   |
/// |1| |3| |5| |7| |

pub struct Fw<G: Monoid>(Vec<G::S>);

impl<G> Fw<G>
where
    G: Monoid + Commutative,
    G::S: Clone,
{
    /// you need to pass initial values because,
    /// it might not be identity element.

    pub fn new(a: Vec<G::S>) -> Self {
        let n = a.len();

        let mut node = vec![G::e()];

        node.append(&mut a.to_vec());

        for i in 1..n {
            let j = i + (1 << i.trailing_zeros());

            if j <= n {
                node[j] = G::op(node[j].clone(), node[i].clone());
            }
        }

        Self(node)
    }

    pub fn size(&self) -> usize { self.0.len() - 1 }

    /// a[i] += v

    pub fn operate(
        &mut self,
        mut i: usize,
        v: G::S,
    ) {
        i += 1;

        while i <= self.size() {
            self.0[i] = G::op(self.0[i].clone(), v.clone());

            i += 1 << i.trailing_zeros();
        }
    }

    // \sum_{j=0}^{i-1} a[i].
    pub fn fold_lt(
        &self,
        mut i: usize,
    ) -> G::S {
        let mut v = G::e();

        while i > 0 {
            v = G::op(v, self.0[i].clone());

            i -= 1 << i.trailing_zeros();
        }

        v
    }

    /// max i (l < i <= n) satisfying f(op(v, fold_lt(i))) is true.
    /// f(op(v, fold_lt(i))) must be monotonous for i.
    /// l(true, .., true, false, .., false]n
    /// if l == n or f(op(v, fold_lt(l + 1))) is false, return l.

    fn _max_right<F>(
        &self,
        f: &F,
        l: usize,
        mut v: G::S,
    ) -> usize
    where
        F: Fn(&G::S) -> bool,
    {
        let n = self.size();

        let mut d = (n + 1).next_power_of_two(); // covering
        let mut i = 0;

        loop {
            d >>= 1;

            if d == 0 {
                debug_assert!(l <= i && i <= n);

                return i;
            }

            if i + d > n {
                continue;
            }

            let nv = G::op(v.clone(), self.0[i + d].clone());

            if i + d <= l || f(&nv) {
                i += d;

                v = nv;
            }
        }
    }

    /// max i satisfying f(fold_lt(i)) is true.
    /// f(fold_lt(i)) must be monotonous for i. [tr, .., tr, fal, .., fal]

    pub fn max_right<F>(
        &self,
        f: &F,
    ) -> usize
    where
        F: Fn(&G::S) -> bool,
    {
        self._max_right(f, 0, G::e())
    }
}

impl<G> Fw<G>
where
    G: AbelianGroup,
    G::S: Clone,
{
    /// get range [l, r) = fold_lt(r) - fold_lt(l)

    pub fn fold(
        &self,
        l: usize,
        r: usize,
    ) -> G::S {
        assert!(l <= r);

        G::op(G::inv(self.fold_lt(l)), self.fold_lt(r))
    }

    /// max i (l < i <= n) f(fold(l, i)) is true. l(tr, .., tr, fal, .. fal]n
    /// or l

    pub fn max_right_from<F>(
        &self,
        f: &F,
        l: usize,
    ) -> usize
    where
        F: Fn(&G::S) -> bool,
    {
        self._max_right(f, l, G::inv(self.fold_lt(l)))
    }

    /// min i (0 <= i < r), f(fold(i, r)) is true. 0[fal, .. fal, tr, .. tr)r
    /// or r

    pub fn min_left_from<F>(
        &self,
        f: &F,
        r: usize,
    ) -> usize
    where
        F: Fn(&G::S) -> bool,
    {
        let n = self.size();

        assert!(r <= n);

        if r == 0 {
            return 0;
        }

        let mut d = (n + 1).next_power_of_two();

        let mut v = self.fold_lt(r);

        if f(&v) {
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

            let nv = G::op(G::inv(self.0[i + d - 1].clone()), v.clone());

            if !f(&nv) {
                i += d;

                v = nv;
            }
        }
    }
}

pub struct Dual<G: Monoid>(Fw<G>);

impl<G> Dual<G>
where
    G: Monoid + Commutative,
    G::S: Clone,
{
    pub fn new(mut a: Vec<G::S>) -> Self
    where
        G: Inverse,
        G::S: Clone,
    {
        for i in (1..a.len()).rev() {
            a[i] = G::op(G::inv(a[i - 1].clone()), a[i].clone());
        }

        Self(Fw::new(a))
    }

    pub fn size(&self) -> usize { self.0.size() }

    /// a[i] += v (l <= i < n)

    pub fn operate_ge(
        &mut self,
        i: usize,
        v: G::S,
    ) {
        self.0.operate(i, v)
    }

    /// a[i]

    pub fn get(
        &self,
        i: usize,
    ) -> G::S {
        self.0.fold_lt(i + 1)
    }

    /// find first index i satisfying
    /// `is_ok(&self.get_point(i)) == true`
    /// Constraints:
    /// `is_ok(&self.get_point(i))` must be monotonous [false,
    /// false, .., true, true] if such an i is not found,
    /// return `self.size()`

    pub fn search<F>(
        &self,
        is_ok: &F,
    ) -> usize
    where
        F: Fn(&G::S) -> bool,
    {
        self.0.max_right(&|prod: &G::S| !is_ok(prod))
    }
}

impl<G> Dual<G>
where
    G: AbelianGroup,
    G::S: Clone,
{
    /// a[i] += v (l <= i < r)

    pub fn operate(
        &mut self,
        l: usize,
        r: usize,
        v: G::S,
    ) {
        assert!(l < r && r <= self.size());

        self.operate_ge(l, v.clone());

        if r < self.size() {
            self.operate_ge(r, G::inv(v));
        }
    }

    /// prod[left, index) >= target_value - prod[0, left)
    /// prod[left, index) + prod[0, left) >= target_value
    /// is_ok(G::operate_ge(prod[left, index), prod[0, left)))
    /// `is_ok`'s results must be mnotonous
    /// in the range of [left, self.size())
    /// [?, .., ?, false(left), .., false, true .., true]
    /// where first false index corresponds
    /// to the given left, it might be there exists no
    /// false.

    pub fn search_from<F>(
        &self,
        is_ok: &F,
        l: usize,
    ) -> usize
    where
        F: Fn(&G::S) -> bool,
    {
        assert!(l <= self.size());

        let prod_lt = if l == 0 { G::e() } else { self.get(l - 1) };

        self.0.max_right_from(
            &|prod_ge: &G::S| !is_ok(&G::op(prod_lt.clone(), prod_ge.clone())),
            l,
        )
    }

    // /// [false, .. false, true, .., true, ?, .. ?]
    // /// find first true index.
    // /// no longer necessary function.
    // pub fn binary_search_from_right<F>(&self, is_ok: &F, right:
    // usize) -> usize where
    //     F: Fn(&S) -> bool,
    // {
    //     assert!(right <= self.size());
    // }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_fw() {
        use crate::{
            algebraic_structure_impl::GroupApprox,
            group_theory_id::Additive,
        };

        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut fw = Fw::<GroupApprox<i32, Additive>>::new(arr);

        assert_eq!(fw.fold(0, 10), 45);

        assert_eq!(fw.fold(6, 10), 30);

        fw.operate(5, 10);

        assert_eq!(fw.fold(6, 10), 30);

        assert_eq!(fw.fold_lt(5), 10);

        assert_eq!(fw.fold_lt(6), 25);

        assert_eq!(fw.fold(5, 6), 15);

        let is_ok = |x: &i32| *x <= 25;

        assert_eq!(fw.max_right(&is_ok), 6);

        assert_eq!(fw.max_right_from(&is_ok, 0), 6);

        let is_ok = |x: &i32| *x < 25;

        assert_eq!(fw.max_right(&is_ok), 5);

        assert_eq!(fw.max_right_from(&is_ok, 0), 5);

        assert_eq!(fw.max_right_from(&is_ok, 4), 6);

        assert_eq!(fw.max_right_from(&is_ok, 5), 7);

        assert_eq!(fw.max_right_from(&is_ok, 6), 9);

        assert_eq!(fw.max_right_from(&is_ok, 9), 10);

        assert_eq!(fw.min_left_from(&is_ok, 10), 7);

        assert_eq!(fw.min_left_from(&is_ok, 0), 0);

        assert_eq!(fw.min_left_from(&is_ok, 6), 2);

        assert_eq!(fw.min_left_from(&is_ok, 5), 0);

        let is_ok = |x: &i32| *x < 15;

        assert_eq!(fw.max_right_from(&is_ok, 5), 5);

        assert_eq!(fw.min_left_from(&is_ok, 6), 6);

        assert_eq!(fw.min_left_from(&is_ok, 10), 9);

        let is_ok = |x: &i32| *x < 9;

        assert_eq!(fw.min_left_from(&is_ok, 10), 10);
    }

    #[test]

    fn test_dual() {
        use crate::{
            algebraic_structure_impl::GroupApprox,
            group_theory_id::Additive,
        };

        let mut a = (0..10).collect::<Vec<_>>();

        for i in 0..9 {
            a[i + 1] += a[i];
        }

        type Fw = Dual<GroupApprox<i32, Additive>>;

        let mut fw = Fw::new(a);

        assert_eq!(fw.get(1), 1);

        assert_eq!(fw.get(5), 15);

        assert_eq!(fw.get(9), 45);

        fw.operate_ge(5, 2);

        assert_eq!(fw.get(1), 1);

        assert_eq!(fw.get(5), 17);

        assert_eq!(fw.get(9), 47);

        assert_eq!(fw.search(&|value: &i32| *value >= 23), 6);

        assert_eq!(fw.search(&|value: &i32| *value >= 47), 9);

        assert_eq!(fw.search(&|value: &i32| *value > 47), 10);

        fw.operate(2, 6, 1);

        assert_eq!(fw.get(1), 1);

        assert_eq!(fw.get(5), 18);

        assert_eq!(fw.get(9), 47);

        fw.operate(2, 6, -1);

        assert_eq!(fw.search_from(&|value: &i32| *value >= 23, 0), 6);

        assert_eq!(fw.search_from(&|value: &i32| *value >= 47, 0), 9);

        assert_eq!(fw.search_from(&|value: &i32| *value > 47, 0), 10);

        let b = (0..10).map(|i| fw.get(i)).collect::<Vec<_>>();

        assert_eq!(b, [0, 1, 3, 6, 10, 17, 23, 30, 38, 47]);

        assert_eq!(fw.search_from(&|value: &i32| *value >= 23, 7), 7);

        assert_eq!(fw.search_from(&|value: &i32| *value >= 23, 5), 6);
    }
}
