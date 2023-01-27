use std::iter::FromIterator;

use crate::algebraic_structure::*;

/// Segment Tree

pub struct Segtree<M: Monoid> {
    pub(crate) size: usize,
    pub(crate) node: Vec<M::S>,
}

impl<M> std::iter::FromIterator<M::S> for Segtree<M>
where
    M: Monoid,
    M::S: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let mut node = iter.into_iter().collect::<Vec<_>>();

        let size = node.len();

        let n = size.next_power_of_two();

        node = (0..n)
            .map(|_| M::e())
            .chain(node.into_iter())
            .chain((0..n - size).map(|_| M::e()))
            .collect::<Vec<_>>();

        let mut seg = Self { size, node };

        (1..n).rev().for_each(|i| seg.update(i));

        seg
    }
}

impl<M: Monoid> Segtree<M> {
    pub fn size(&self) -> usize { self.size }

    pub(crate) fn n(&self) -> usize { self.node.len() >> 1 }
}

impl<M> Segtree<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new<F>(
        size: usize,
        default: F,
    ) -> Self
    where
        F: Fn() -> M::S,
    {
        Self::from_iter((0..size).map(|_| default()))
    }

    fn update(
        &mut self,
        i: usize,
    ) {
        self.node[i] =
            M::op(self.node[i << 1].clone(), self.node[i << 1 | 1].clone());
    }

    pub fn set(
        &mut self,
        mut i: usize,
        x: M::S,
    ) {
        assert!(i < self.size);

        i += self.n();

        self.node[i] = x;

        while i > 1 {
            i >>= 1;

            self.update(i);
        }
    }

    /// why `reduce` but `fold`?
    /// but initial element internally is just the identity element.
    /// it's not an arbitrary element.
    /// also, it is not necessarily used to compute _{op}\prod_l^r (l < r).
    /// we use the identity only to make it easy implementation.
    /// (requireing monoid for simplicity,
    /// however, strictly, it's enough to be only semigrouop.)
    /// so this method should be called `reduce`.

    pub fn reduce(
        &self,
        mut l: usize,
        mut r: usize,
    ) -> M::S {
        assert!(l < r && r <= self.size);

        let n = self.n();

        l += n;

        r += n;

        let mut vl = M::e();

        let mut vr = M::e();

        while l < r {
            if l & 1 == 1 {
                vl = M::op(vl, self.node[l].clone());

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                vr = M::op(self.node[r].clone(), vr);
            }

            l >>= 1;

            r >>= 1;
        }

        M::op(vl, vr)
    }
}

impl<M> Segtree<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn reduce_recurse(
        &self,
        l: usize,
        r: usize,
    ) -> M::S {
        assert!(l <= r && r <= self.size);

        self._reduce_recurse(l, r, 0, self.n(), 1)
    }

    fn _reduce_recurse(
        &self,
        l: usize,
        r: usize,
        cur_l: usize,
        cur_r: usize,
        i: usize,
    ) -> M::S {
        if cur_r <= l || r <= cur_l {
            return M::e();
        }

        if l <= cur_l && cur_r <= r {
            return self.node[i].clone();
        }

        let c = (cur_l + cur_r) >> 1;

        M::op(
            self._reduce_recurse(l, r, cur_l, c, i << 1),
            self._reduce_recurse(l, r, c, cur_r, i << 1 | 1),
        )
    }
}

/// indexing

impl<M> std::ops::Index<usize> for Segtree<M>
where
    M: Monoid,
{
    type Output = M::S;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        assert!(i < self.size());

        &self.node[i + self.n()]
    }
}

impl<M> From<&[M::S]> for Segtree<M>
where
    M: Monoid,
    M::S: Clone,
{
    fn from(slice: &[M::S]) -> Self { Self::from_iter(slice.iter().cloned()) }
}

impl<M> Segtree<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn max_right<F>(
        &self,
        is_ok: &F,
        l: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.size);

        if l == self.size {
            return self.size;
        }

        let n = self.n();

        let mut v = M::e();

        let mut i = l + n;

        debug_assert_ne!(i, 0);

        loop {
            i >>= i.trailing_zeros(); // upstream
            let nv = M::op(v.clone(), self.node[i].clone());

            if !is_ok(&nv) {
                break;
            }

            // otherwise up one stair to right
            i += 1;

            v = nv;

            if i.count_ones() == 1 {
                return self.size;
            }
        }

        // down stairs to right
        while i < n {
            i <<= 1;

            let nv = M::op(v.clone(), self.node[i].clone());

            if !is_ok(&nv) {
                continue;
            }

            v = nv;

            i += 1;
        }

        i - n
    }

    pub fn min_left<F>(
        &self,
        is_ok: &F,
        r: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.size);

        if r == 0 {
            return 0;
        }

        let n = self.n();

        let mut v = M::e();

        let mut i = r + n;

        debug_assert_ne!(i, 0);

        loop {
            i >>= i.trailing_zeros(); // upstream
            let nv = M::op(self.node[i - 1].clone(), v.clone());

            if !is_ok(&nv) {
                break;
            }

            i -= 1;

            v = nv;

            if i.count_ones() == 1 {
                return 0;
            }
        }

        while i < n {
            i <<= 1;

            let nv = M::op(self.node[i - 1].clone(), v.clone());

            if !is_ok(&nv) {
                continue;
            }

            i -= 1;

            v = nv;
        }

        i - n
    }
}

impl<M> Segtree<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn max_right_recurse<F>(
        &self,
        is_ok: &F,
        l: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.size);

        self._max_right_recurse(is_ok, l, 0, self.n(), &mut M::e(), 1)
    }

    /// find max right satisfying current_left <= right <= current_right.
    /// if current_right <= left, return left
    /// if current_left >= self.size, return self.size

    fn _max_right_recurse<F>(
        &self,
        is_ok: &F,
        l: usize,
        cur_l: usize,
        cur_r: usize,
        v: &mut M::S,
        i: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        if cur_r <= l {
            return l;
        }

        if cur_l >= self.size {
            return self.size;
        }

        let nv = M::op(v.clone(), self.node[i].clone());

        if l <= cur_l && cur_r <= self.size && is_ok(&nv) {
            *v = nv;

            return cur_r;
        }

        if cur_r - cur_l == 1 {
            return cur_l;
        }

        let c = (cur_l + cur_r) >> 1;

        let res = self._max_right_recurse(is_ok, l, cur_l, c, v, i << 1);

        if res < c || res == self.size {
            return res;
        }

        self._max_right_recurse(is_ok, l, c, cur_r, v, i << 1 | 1)
    }

    pub fn min_left_recurse<F>(
        &self,
        is_ok: &F,
        r: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.size);

        self._min_left_recurse(is_ok, r, 0, self.n(), &mut M::e(), 1)
    }

    fn _min_left_recurse<F>(
        &self,
        is_ok: &F,
        r: usize,
        cur_l: usize,
        cur_r: usize,
        v: &mut M::S,
        i: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        if cur_l >= r {
            return r;
        }

        let nv = M::op(self.node[i].clone(), v.clone());

        if cur_r <= r && is_ok(&nv) {
            *v = nv;

            return cur_l;
        }

        if cur_r - cur_l == 1 {
            return cur_r;
        }

        let c = (cur_l + cur_r) >> 1;

        let res = self._min_left_recurse(is_ok, r, c, cur_r, v, i << 1 | 1);

        if res > c {
            return res;
        }

        self._min_left_recurse(is_ok, r, cur_l, c, v, i << 1)
    }
}

use crate::{
    algebraic_structure_impl::*,
    query::RangeGetQuery,
};

impl<S, I> RangeGetQuery<I> for Segtree<GroupApprox<S, I>>
where
    GroupApprox<S, I>: Monoid<S = S>,
    S: Clone,
{
    type T = S;

    fn get_range(
        &mut self,
        l: usize,
        r: usize,
    ) -> Self::T {
        self.reduce(l, r)
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use crate::group_theory_id::Additive;

    type Seg = Segtree<GroupApprox<usize, Additive>>;

    #[test]

    fn test_basic() {
        let mut seg = Seg::new(10, || 0);

        assert_eq!(seg.reduce(0, 10), 0);

        seg.set(5, 5);

        assert_eq!(seg.reduce(0, 10), 5);

        seg.set(5, 10);

        assert_eq!(seg.reduce(0, 10), 10);
    }

    #[test]

    fn test_indexing() {
        let mut seg = Seg::new(10, || 0);

        seg.set(5, 10);

        assert_eq!(seg[5], 10);
    }

    #[test]

    fn test_reduce_recurse() {
        let mut seg = Seg::new(10, || 0);

        assert_eq!(seg.reduce_recurse(0, 10), 0);

        seg.set(5, 5);

        assert_eq!(seg.reduce_recurse(0, 10), 5);

        seg.set(5, 10);

        assert_eq!(seg.reduce_recurse(0, 10), 10);
    }

    #[test]

    fn test_binary_search() {
        // use crate::monoid::Monoid;
        let mut seg = Seg::new(10, || 0);

        assert_eq!(seg.reduce(0, 10), 0);

        seg.set(5, 10);

        let is_ok = &|sum: &usize| *sum < 10;

        assert_eq!(seg.max_right(is_ok, 0), 5);

        assert_eq!(seg.max_right(is_ok, 10), 10);

        assert_eq!(seg.max_right(is_ok, 5), 5);

        assert_eq!(seg.max_right(is_ok, 6), 10);

        assert_eq!(seg.min_left(is_ok, 10), 6);

        assert_eq!(seg.min_left(is_ok, 5), 0);

        assert_eq!(seg.min_left(is_ok, 6), 6);
    }

    #[test]

    fn test_binary_search_recurse() {
        let mut seg = Seg::new(10, || 0);

        assert_eq!(seg.reduce(0, 10), 0);

        seg.set(5, 10);

        let is_ok = &|sum: &usize| *sum < 10;

        assert_eq!(seg.max_right_recurse(is_ok, 0), 5);

        assert_eq!(seg.max_right_recurse(is_ok, 10), 10);

        assert_eq!(seg.max_right_recurse(is_ok, 5), 5);

        assert_eq!(seg.max_right_recurse(is_ok, 6), 10);

        assert_eq!(seg.min_left_recurse(is_ok, 10), 6);

        assert_eq!(seg.min_left_recurse(is_ok, 5), 0);

        assert_eq!(seg.min_left_recurse(is_ok, 6), 6);
    }
}
