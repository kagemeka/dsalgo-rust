use crate::{
    algebraic_structure::*,
    binary_function::*,
    bit_length_with_count_leading_zeros_u64::bit_length,
};

#[derive(Debug)]

pub struct SegtreeLazy<Sg, Fg, M>
where
    Sg: Monoid,
    Fg: Monoid,
    M: BinaryFunc<L = Fg::S, R = Sg::S, Cod = Sg::S>,
{
    size: usize,               // size
    h: u8,                     // height
    pub(crate) d: Vec<Sg::S>,  // data, pub(crate) for debug
    pub(crate) lz: Vec<Fg::S>, // lazy operators
    _phantom: std::marker::PhantomData<M>,
}

impl<Sg, Fg, M> SegtreeLazy<Sg, Fg, M>
where
    Sg: Monoid,
    Fg: Monoid,
    M: BinaryFunc<L = Fg::S, R = Sg::S, Cod = Sg::S>,
    Sg::S: Clone,
    Fg::S: Clone,
{
    pub fn new(a: Vec<Sg::S>) -> Self {
        let size = a.len();

        assert!(size > 0);

        let n = a.len().next_power_of_two();

        let mut d = vec![Sg::e(); n << 1];

        d[n..(n + size)].clone_from_slice(&a);

        let mut seg = Self {
            size,
            h: bit_length(n as u64),
            d,
            lz: vec![Fg::e(); n],
            _phantom: std::marker::PhantomData,
        };

        for i in (1..n).rev() {
            seg.update(i);
        }

        seg
    }

    pub fn size(&self) -> usize { self.size }

    fn n(&self) -> usize { self.d.len() >> 1 }

    /// merge child values and replace with it.

    fn update(
        &mut self,
        i: usize,
    ) {
        self.d[i] = Sg::op(self.d[i << 1].clone(), self.d[i << 1 | 1].clone());
    }

    /// apply operator f to node i.

    fn apply_node(
        &mut self,
        i: usize,
        f: Fg::S,
    ) {
        self.d[i] = M::f(f.clone(), self.d[i].clone());

        if i < self.lz.len() {
            self.lz[i] = Fg::op(f, self.lz[i].clone());
        }
    }

    /// convey lazy operator to childs

    fn propagate(
        &mut self,
        i: usize,
    ) {
        let f = self.lz[i].clone();

        self.apply_node(i << 1, f.clone());

        self.apply_node(i << 1 | 1, f);

        self.lz[i] = Fg::e();
    }

    // from root
    fn propagate_above(
        &mut self,
        i: usize,
    ) {
        for j in (1..self.h).rev() {
            self.propagate(i >> j);
        }
    }

    // from leaf
    fn update_above(
        &mut self,
        mut i: usize,
    ) {
        while i > 1 {
            i >>= 1;

            self.update(i);
        }
    }

    pub fn apply(
        &mut self,
        mut l: usize,
        mut r: usize,
        f: Fg::S,
    ) {
        assert!(l <= r && r <= self.size);

        let n = self.n();

        l += n;

        r += n;

        let l0 = l >> l.trailing_zeros();

        let r0 = (r >> r.trailing_zeros()) - 1;

        self.propagate_above(l0);

        self.propagate_above(r0);

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

        self.update_above(l0);

        self.update_above(r0);
    }

    pub fn get(
        &mut self,
        mut i: usize,
    ) -> Sg::S {
        assert!(i < self.size);

        i += self.n();

        self.propagate_above(i);

        self.d[i].clone()
    }

    pub fn apply_point<F>(
        &mut self,
        i: usize,
        f: F,
    ) where
        F: Fn(Sg::S) -> Sg::S,
    {
        let n = self.n();

        self.d[i + n] = f(self.get(i));

        self.update_above(i + n);
    }

    pub fn set(
        &mut self,
        i: usize,
        v: Sg::S,
    ) {
        self.apply_point(i, |_| v.clone());
    }

    pub fn reduce(
        &mut self,
        mut l: usize,
        mut r: usize,
    ) -> Sg::S {
        assert!(l <= r && r <= self.size);

        let n = self.n();

        l += n;

        r += n;

        self.propagate_above(l);

        self.propagate_above(r - 1);

        let mut vl = Sg::e();

        let mut vr = Sg::e();

        while l < r {
            if l & 1 == 1 {
                vl = Sg::op(vl, self.d[l].clone());

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                vr = Sg::op(self.d[r].clone(), vr);
            }

            l >>= 1;

            r >>= 1;
        }

        Sg::op(vl, vr)
    }

    /// prod[l, r) = true

    pub fn max_right<G>(
        &mut self,
        is_ok: &G,
        l: usize,
    ) -> usize
    where
        G: Fn(&Sg::S) -> bool,
    {
        assert!(l <= self.size);

        if l == self.size {
            return self.size;
        }

        let n = self.n();

        let mut v = Sg::e();

        let mut i = n + l;

        self.propagate_above(i);

        loop {
            i >>= i.trailing_zeros();

            let nv = Sg::op(v.clone(), self.d[i].clone());

            if !is_ok(&nv) {
                break;
            }

            // up one stair from left
            v = nv;

            i += 1;

            if i.count_ones() == 1 {
                // wall.
                return self.size;
            }
        }

        // down stairs to right
        while i < n {
            self.propagate(i);

            i <<= 1;

            let nv = Sg::op(v.clone(), self.d[i].clone());

            if !is_ok(&nv) {
                continue;
            }

            v = nv;

            i += 1;
        }

        i - n
    }

    /// prod[l, r) = true

    pub fn min_left<G>(
        &mut self,
        is_ok: &G,
        r: usize,
    ) -> usize
    where
        G: Fn(&Sg::S) -> bool,
    {
        assert!(r <= self.size);

        if r == 0 {
            return 0;
        }

        let n = self.n();

        let mut v = Sg::e();

        let mut i = n + r;

        self.propagate_above(i - 1);

        loop {
            debug_assert!(i >= 1);

            i >>= i.trailing_zeros();

            let nv = Sg::op(self.d[i - 1].clone(), v.clone());

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

            let nv = Sg::op(self.d[i - 1].clone(), v.clone());

            if !is_ok(&nv) {
                continue;
            }

            i -= 1;

            v = nv;
        }

        i - n
    }

    pub fn apply_recurse(
        &mut self,
        l: usize,
        r: usize,
        f: Fg::S,
    ) {
        assert!(l <= r && r <= self.size);

        self._apply_recurse(l, r, 0, self.n(), 1, f);
    }

    fn _apply_recurse(
        &mut self,
        l: usize,
        r: usize,
        cl: usize, // current left
        cr: usize, // current right
        i: usize,  // current node
        f: Fg::S,
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

        self._apply_recurse(l, r, cl, c, i << 1, f.clone());

        self._apply_recurse(l, r, c, cr, i << 1 | 1, f);

        self.update(i);
    }

    pub fn reduce_recurse(
        &mut self,
        l: usize,
        r: usize,
    ) -> Sg::S {
        assert!(l <= r && r <= self.size);

        self._reduce_recurse(l, r, 0, self.n(), 1)
    }

    fn _reduce_recurse(
        &mut self,
        l: usize,
        r: usize,
        cl: usize,
        cr: usize,
        i: usize,
    ) -> Sg::S {
        if cr <= l || r <= cl {
            return Sg::e();
        }

        if l <= cl && cr <= r {
            return self.d[i].clone();
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        Sg::op(
            self._reduce_recurse(l, r, cl, c, i << 1),
            self._reduce_recurse(l, r, c, cr, i << 1 | 1),
        )
    }

    pub fn max_right_recurse<G>(
        &mut self,
        is_ok: &G,
        l: usize,
    ) -> usize
    where
        G: Fn(&Sg::S) -> bool,
    {
        assert!(l <= self.size);

        self._max_right_recurse(is_ok, l, 0, self.n(), &mut Sg::e(), 1)
    }

    fn _max_right_recurse<G>(
        &mut self,
        is_ok: &G,
        l: usize,
        cl: usize,
        cr: usize,
        v: &mut Sg::S,
        i: usize,
    ) -> usize
    where
        G: Fn(&Sg::S) -> bool,
    {
        if cr <= l {
            return l;
        }

        if self.size <= cl {
            return self.size;
        }

        let nv = Sg::op(v.clone(), self.d[i].clone());

        if l <= cl && cr <= self.size && is_ok(&nv) {
            *v = nv;

            return cr;
        }

        if cr - cl == 1 {
            return cl;
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        let r = self._max_right_recurse(is_ok, l, cl, c, v, i << 1);

        if r < c || r == self.size {
            return r;
        }

        self._max_right_recurse(is_ok, l, c, cr, v, i << 1 | 1)
    }

    pub fn min_left_recurse<G>(
        &mut self,
        is_ok: &G,
        r: usize,
    ) -> usize
    where
        G: Fn(&Sg::S) -> bool,
    {
        assert!(r <= self.size);

        self._min_left_recurse(is_ok, r, 0, self.n(), &mut Sg::e(), 1)
    }

    fn _min_left_recurse<G>(
        &mut self,
        is_ok: &G,
        r: usize,
        cl: usize,
        cr: usize,
        v: &mut Sg::S,
        i: usize,
    ) -> usize
    where
        G: Fn(&Sg::S) -> bool,
    {
        if cl >= r {
            return r;
        }

        let nv = Sg::op(self.d[i].clone(), v.clone());

        if cr <= r && is_ok(&nv) {
            *v = nv;

            return cl;
        }

        if cr - cl == 1 {
            return cr;
        }

        self.propagate(i);

        let c = (cl + cr) >> 1;

        let l = self._min_left_recurse(is_ok, r, c, cr, v, i << 1 | 1);

        if l > c || l == 0 {
            return l;
        }

        self._min_left_recurse(is_ok, r, cl, c, v, i << 1)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        #[derive(Clone, PartialEq, Debug, Copy)]

        struct Data {
            pub sum: i32,
            pub len: usize,
        }

        struct RARS<T>(std::marker::PhantomData<T>);

        impl BinaryOp for RARS<Data> {
            type S = Data;

            fn op(
                a: Self::S,
                b: Self::S,
            ) -> Self::S {
                Data { sum: a.sum + b.sum, len: a.len + b.len }
            }
        }

        impl Associative for RARS<Data> {}

        impl Identity for RARS<Data> {
            fn e() -> Self::S { Data { sum: 0, len: 0 } }
        }

        use crate::{
            algebraic_structure_impl::GroupApprox,
            group_theory_id::Additive,
        };

        struct Map;

        impl BinaryFunc for RARS<Map> {
            type Cod = Data;

            type L = i32;

            type R = Data;

            fn f(
                op: Self::L,
                x: Self::R,
            ) -> Self::Cod {
                Data { sum: x.sum + op * x.len as i32, len: x.len }
            }
        }

        let a = vec![Data { sum: 0, len: 1 }; 10];

        let mut seg = SegtreeLazy::<
            RARS<Data>,
            GroupApprox<i32, Additive>,
            RARS<Map>,
        >::new(a);

        assert_eq!(seg.reduce(0, 10), Data { sum: 0, len: 10 });

        seg.apply(0, 5, 2);

        // [2, 2, 2, 2, 2, 0, 0, 0, 0, 0]
        assert_eq!(seg.reduce(2, 6), Data { sum: 6, len: 4 });

        assert_eq!(seg.reduce_recurse(2, 6), Data { sum: 6, len: 4 });

        assert_eq!(seg.reduce(0, 10), Data { sum: 10, len: 10 });

        assert_eq!(seg.reduce_recurse(0, 10), Data { sum: 10, len: 10 });

        assert_eq!(seg.max_right(&|x: &Data| x.sum <= 3, 4), 10);

        assert_eq!(seg.max_right_recurse(&|x: &Data| x.sum <= 3, 4), 10);

        assert_eq!(seg.max_right(&|x: &Data| x.sum <= 3, 2), 3);

        assert_eq!(seg.max_right_recurse(&|x: &Data| x.sum <= 3, 2), 3);

        assert_eq!(seg.min_left(&|x: &Data| x.sum <= 3, 4), 3);

        assert_eq!(seg.min_left_recurse(&|x: &Data| x.sum <= 3, 4), 3);

        assert_eq!(seg.min_left(&|x: &Data| x.sum <= 3, 10), 4);

        assert_eq!(seg.min_left_recurse(&|x: &Data| x.sum <= 3, 10), 4);

        assert_eq!(seg.min_left(&|x: &Data| x.sum < 0, 10), 10);

        assert_eq!(seg.min_left_recurse(&|x: &Data| x.sum < 0, 10), 10);

        seg.set(2, Data { sum: -1, len: 1 });

        // [2, 2, -1, 2, 2, 0, 0, 0, 0, 0]
        assert_eq!(seg.reduce(0, 10), Data { sum: 7, len: 10 });

        assert_eq!(seg.reduce_recurse(0, 10), Data { sum: 7, len: 10 });

        seg.apply_recurse(1, 7, 3);

        // [2, 5, 2, 5, 5, 3, 3, 0, 0, 0]
        assert_eq!(seg.get(4), Data { sum: 5, len: 1 });

        assert_eq!(seg.reduce(0, 10), Data { sum: 25, len: 10 });
    }
}
