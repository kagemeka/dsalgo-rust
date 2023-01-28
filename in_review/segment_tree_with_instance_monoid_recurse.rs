pub trait Monoid {
    type T;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;
}

pub struct Segtree<G: Monoid> {
    g: G,
    size: usize,
    data: Vec<G::T>,
}

impl<G: Monoid> Segtree<G> {
    fn n(&self) -> usize { self.data.len() >> 1 }
}

use std::ops::*;

impl<G: Monoid> Index<usize> for Segtree<G> {
    type Output = G::T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.data[i + self.n()]
    }
}

impl<G: Monoid> Segtree<G>
where
    G::T: Clone,
{
    pub fn size(&self) -> usize { self.size }

    fn merge(
        &mut self,
        i: usize,
    ) {
        self.data[i] =
            self.g.op(self.data[i << 1].clone(), self.data[i << 1 | 1].clone());
    }

    pub fn new(
        g: G,
        size: usize,
    ) -> Self {
        assert!(size > 0);

        let n = size.next_power_of_two();

        let data = vec![g.e(); n << 1];

        Self { g, size, data }
    }

    pub fn set(
        &mut self,
        i: usize,
        x: G::T,
    ) {
        assert!(i < self.size());

        self._set(i, 0, self.n(), 1, x);
    }

    fn _set(
        &mut self,
        i: usize,
        cl: usize,
        cr: usize,
        ci: usize,
        x: G::T,
    ) {
        assert!(cl <= i && i < cr);

        if cr - cl == 1 {
            self.data[ci] = x;

            return;
        }

        let c = (cl + cr) >> 1;

        if i < c {
            self._set(i, cl, c, ci << 1, x);
        } else {
            self._set(i, c, cr, ci << 1 | 1, x);
        }

        self.merge(ci);
    }

    pub fn fold(
        &mut self,
        l: usize,
        r: usize,
    ) -> G::T {
        assert!(l <= r && r <= self.size);

        self._fold(l, r, 0, self.n(), 1)
    }

    fn _fold(
        &mut self,
        l: usize,
        r: usize,
        cl: usize,
        cr: usize,
        i: usize,
    ) -> G::T {
        if cr <= l || r <= cl {
            return self.g.e();
        }

        if l <= cl && cr <= r {
            return self.data[i].clone();
        }

        let c = (cl + cr) >> 1;

        let vl = self._fold(l, r, cl, c, i << 1);

        let vr = self._fold(l, r, c, cr, i << 1 | 1);

        self.g.op(vl, vr)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        struct G;

        impl Monoid for G {
            type T = i64;

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

        let mut seg = Segtree::new(G {}, n);

        seg.set(2, 1);

        assert_eq!(seg[2], 1);

        assert_eq!(seg.fold(0, 5), 1);
    }
}
