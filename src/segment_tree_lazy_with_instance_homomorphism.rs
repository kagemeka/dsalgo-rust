use crate::bit_length_with_count_leading_zeros_usize::bit_length;

pub trait Ops {
    type S;

    type F;

    fn op(
        &self,
        a: Self::S,
        b: Self::S,
    ) -> Self::S;

    fn e(&self) -> Self::S;

    fn compose(
        &self,
        f: Self::F,
        g: Self::F,
    ) -> Self::F;

    fn id(&self) -> Self::F;

    fn map(
        &self,
        f: Self::F,
        x: Self::S,
    ) -> Self::S;
}

#[derive(Debug)]

pub struct LazySegtree<O: Ops> {
    ops: O,
    data: Vec<O::S>,
    lazy: Vec<O::F>,
    size: usize,
}

impl<O: Ops> LazySegtree<O>
where
    O::S: Clone,
    O::F: Clone,
{
    pub fn new(
        ops: O,
        size: usize,
    ) -> Self {
        assert!(size > 0);

        let n = size.next_power_of_two();

        let data = vec![ops.e(); n << 1];

        let lazy = vec![ops.id(); n];

        Self { ops, data, lazy, size }
    }

    pub fn size(&self) -> usize { self.size }

    fn n(&self) -> usize { self.lazy.len() }

    fn height(&self) -> usize { bit_length(self.n()) }

    fn merge(
        &mut self,
        i: usize,
    ) {
        self.data[i] = self
            .ops
            .op(self.data[i << 1].clone(), self.data[i << 1 | 1].clone());
    }

    fn apply_node(
        &mut self,
        i: usize,
        f: O::F,
    ) {
        self.data[i] = self.ops.map(f.clone(), self.data[i].clone());

        if i < self.n() {
            self.lazy[i] = self.ops.compose(f, self.lazy[i].clone());
        }
    }

    fn propagate(
        &mut self,
        i: usize,
    ) {
        let f = self.lazy[i].clone();

        self.apply_node(i << 1, f.clone());

        self.apply_node(i << 1 | 1, f);

        self.lazy[i] = self.ops.id();
    }

    fn pull(
        &mut self,
        i: usize,
    ) {
        for j in (1..self.height()).rev() {
            self.propagate(i >> j);
        }
    }

    fn merge_above(
        &mut self,
        mut i: usize,
    ) {
        while i > 1 {
            i >>= 1;

            self.merge(i);
        }
    }

    pub fn set(
        &mut self,
        mut i: usize,
        x: O::S,
    ) {
        assert!(i < self.size);

        i += self.n();

        self.pull(i);

        self.data[i] = x;

        self.merge_above(i);
    }

    pub fn apply(
        &mut self,
        mut l: usize,
        mut r: usize,
        f: O::F,
    ) {
        assert!(l <= r && r <= self.size);

        let n = self.n();

        l += n;

        r += n;

        let l0 = l >> l.trailing_zeros();

        let r0 = (r >> r.trailing_zeros()) - 1;

        self.pull(l0);

        self.pull(r0);

        while l < r {
            if l & 1 == 1 {
                self.apply_node(l, f.clone());

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                self.apply_node(r, f.clone());
            }

            l >>= 1;

            r >>= 1;
        }

        self.merge_above(l0);

        self.merge_above(r0);
    }

    pub fn get(
        &mut self,
        mut i: usize,
    ) -> O::S {
        assert!(i < self.size);

        i += self.n();

        self.pull(i);

        self.data[i].clone()
    }

    pub fn fold(
        &mut self,
        mut l: usize,
        mut r: usize,
    ) -> O::S {
        assert!(l <= r && r <= self.size);

        let n = self.n();

        l += n;

        r += n;

        self.pull(l);

        self.pull(r - 1);

        let mut vl = self.ops.e();

        let mut vr = self.ops.e();

        while l < r {
            if l & 1 == 1 {
                vl = self.ops.op(vl, self.data[l].clone());

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                vr = self.ops.op(self.data[r].clone(), vr);
            }

            l >>= 1;

            r >>= 1;
        }

        self.ops.op(vl, vr)
    }

    pub fn max_right<F>(
        &mut self,
        is_ok: F,
        l: usize,
    ) -> usize
    where
        F: Fn(&O::S) -> bool,
    {
        assert!(l <= self.size);

        if l == self.size {
            return self.size;
        }

        let n = self.n();

        let mut v = self.ops.e();

        let mut i = n + l;

        self.pull(i);

        loop {
            i >>= i.trailing_zeros();

            let nv = self.ops.op(v.clone(), self.data[i].clone());

            if !is_ok(&nv) {
                break;
            }

            v = nv;

            i += 1;

            if i.count_ones() == 1 {
                return self.size;
            }
        }

        while i < n {
            self.propagate(i);

            i <<= 1;

            let nv = self.ops.op(v.clone(), self.data[i].clone());

            if !is_ok(&nv) {
                continue;
            }

            v = nv;

            i += 1;
        }

        i - n
    }

    pub fn min_left<F>(
        &mut self,
        is_ok: F,
        r: usize,
    ) -> usize
    where
        F: Fn(&O::S) -> bool,
    {
        assert!(r <= self.size);

        if r == 0 {
            return 0;
        }

        let n = self.n();

        let mut v = self.ops.e();

        let mut i = n + r;

        self.pull(i - 1);

        loop {
            debug_assert!(i >= 1);

            i >>= i.trailing_zeros();

            let nv = self.ops.op(self.data[i - 1].clone(), v.clone());

            if !is_ok(&nv) {
                break;
            }

            i -= 1;

            v = nv;

            if i == 0 || i.count_ones() == 1 {
                return 0;
            }
        }

        while i < n {
            debug_assert!(i >= 1);

            self.propagate(i - 1);

            i <<= 1;

            let nv = self.ops.op(self.data[i - 1].clone(), v.clone());

            if !is_ok(&nv) {
                continue;
            }

            i -= 1;

            v = nv;
        }

        i - n
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        struct Lz;

        impl Ops for Lz {
            type F = i64;

            type S = (i64, usize);

            fn op(
                &self,
                a: Self::S,
                b: Self::S,
            ) -> Self::S {
                (a.0 + b.0, a.1 + b.1)
            }

            fn e(&self) -> Self::S { (0, 0) }

            fn compose(
                &self,
                f: Self::F,
                g: Self::F,
            ) -> Self::F {
                f + g
            }

            fn id(&self) -> Self::F { 0 }

            fn map(
                &self,
                f: Self::F,
                x: Self::S,
            ) -> Self::S {
                (x.0 + x.1 as i64 * f, x.1)
            }
        }

        let n = 5;

        let mut seg = LazySegtree::new(Lz {}, n);

        for i in 0..n {
            seg.set(i, (0, 1));
        }

        for i in 0..n {
            assert_eq!(seg.get(i), (0, 1));
        }

        seg.apply(1, 3, 1);

        assert_eq!(seg.fold(0, n), (2, 5));

        assert_eq!(seg.max_right(|x| x.0 < 2, 0), 2);

        assert_eq!(seg.max_right(|x| x.0 < 2, 2), n);

        assert_eq!(seg.min_left(|x| x.0 < 2, n), 2);

        assert_eq!(seg.min_left(|x| x.0 < 2, 2), 0);
    }
}
