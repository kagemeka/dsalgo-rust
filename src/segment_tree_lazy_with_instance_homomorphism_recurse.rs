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

    pub fn set(
        &mut self,
        i: usize,
        x: O::S,
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
        x: O::S,
    ) {
        assert!(cl <= i && i < cr);

        if cr - cl == 1 {
            self.data[ci] = x;

            return;
        }

        self.propagate(ci);

        let c = (cl + cr) >> 1;

        if i < c {
            self._set(i, cl, c, ci << 1, x);
        } else {
            self._set(i, c, cr, ci << 1 | 1, x);
        }

        self.merge(ci);
    }

    pub fn apply(
        &mut self,
        l: usize,
        r: usize,
        f: O::F,
    ) {
        assert!(l <= r && r <= self.size);

        self._apply(l, r, 0, self.n(), 1, f);
    }

    fn _apply(
        &mut self,
        l: usize,
        r: usize,
        cl: usize,
        cr: usize,
        i: usize,
        f: O::F,
    ) {
        if cr <= l || r <= cl {
            return;
        }

        if l <= cl && cr <= r {
            self.apply_node(i, f);

            return;
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        self._apply(l, r, cl, c, i << 1, f.clone());

        self._apply(l, r, c, cr, i << 1 | 1, f);

        self.merge(i);
    }

    pub fn get(
        &mut self,
        i: usize,
    ) -> O::S {
        self.fold(i, i + 1)
    }

    pub fn fold(
        &mut self,
        l: usize,
        r: usize,
    ) -> O::S {
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
    ) -> O::S {
        if cr <= l || r <= cl {
            return self.ops.e();
        }

        if l <= cl && cr <= r {
            return self.data[i].clone();
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        let vl = self._fold(l, r, cl, c, i << 1);

        let vr = self._fold(l, r, c, cr, i << 1 | 1);

        self.ops.op(vl, vr)
    }

    pub fn max_right<G>(
        &mut self,
        is_ok: G,
        l: usize,
    ) -> usize
    where
        G: Fn(&O::S) -> bool,
    {
        assert!(l <= self.size);

        self._max_right(&is_ok, l, 0, self.n(), &mut self.ops.e(), 1)
    }

    fn _max_right<G>(
        &mut self,
        is_ok: &G,
        l: usize,
        cl: usize,
        cr: usize,
        v: &mut O::S,
        i: usize,
    ) -> usize
    where
        G: Fn(&O::S) -> bool,
    {
        if cr <= l {
            return l;
        }

        if self.size <= cl {
            return self.size;
        }

        let nv = self.ops.op(v.clone(), self.data[i].clone());

        if l <= cl && cr <= self.size && is_ok(&nv) {
            *v = nv;

            return cr;
        }

        if cr - cl == 1 {
            return cl;
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        let r = self._max_right(is_ok, l, cl, c, v, i << 1);

        if r < c || r == self.size {
            return r;
        }

        self._max_right(is_ok, l, c, cr, v, i << 1 | 1)
    }

    pub fn min_left<G>(
        &mut self,
        is_ok: G,
        r: usize,
    ) -> usize
    where
        G: Fn(&O::S) -> bool,
    {
        assert!(r <= self.size);

        self._min_left(&is_ok, r, 0, self.n(), &mut self.ops.e(), 1)
    }

    fn _min_left<G>(
        &mut self,
        is_ok: &G,
        r: usize,
        cl: usize,
        cr: usize,
        v: &mut O::S,
        i: usize,
    ) -> usize
    where
        G: Fn(&O::S) -> bool,
    {
        if cl >= r {
            return r;
        }

        let nv = self.ops.op(self.data[i].clone(), v.clone());

        if cr <= r && is_ok(&nv) {
            *v = nv;

            return cl;
        }

        if cr - cl == 1 {
            return cr;
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        let l = self._min_left(is_ok, r, c, cr, v, i << 1 | 1);

        if l > c || l == 0 {
            return l;
        }

        self._min_left(is_ok, r, cl, c, v, i << 1)
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
