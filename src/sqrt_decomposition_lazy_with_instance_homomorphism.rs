use crate::integer_square_root_with_binary_search_usize::isqrt;

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

pub struct LazySqrtDecomposition<O: Ops> {
    ops: O,
    data: Vec<O::S>,
    buckets: Vec<O::S>,
    lazy: Vec<O::F>,
}

impl<O: Ops> LazySqrtDecomposition<O>
where
    O::S: Clone + PartialEq,
    O::F: Clone + PartialEq,
{
    pub fn size(&self) -> usize { self.data.len() }

    pub fn interval(&self) -> usize {
        let n = self.buckets.len();

        (self.size() + n - 1) / n
    }

    pub fn new(
        ops: O,
        size: usize,
    ) -> Self {
        let data = vec![ops.e(); size];

        let m = isqrt(size);

        let buckets = vec![ops.e(); (size + m - 1) / m];

        let lazy = vec![ops.id(); (size + m - 1) / m];

        Self { ops, data, buckets, lazy }
    }

    fn merge(
        &mut self,
        j: usize,
    ) {
        let m = self.interval();

        let n = self.size();

        self.buckets[j] = self.data[j * m..n.min((j + 1) * m)]
            .iter()
            .cloned()
            .fold(self.ops.e(), |x, y| self.ops.op(x, y))
    }

    fn propagate(
        &mut self,
        j: usize,
    ) {
        let m = self.interval();

        let n = self.size();

        let f = self.lazy[j].clone();

        if f == self.ops.id() {
            return;
        }

        for v in self.data[j * m..n.min((j + 1) * m)].iter_mut() {
            *v = self.ops.map(f.clone(), v.clone());
        }

        self.lazy[j] = self.ops.id();
    }

    fn pull_range(
        &mut self,
        l: usize,
        r: usize,
    ) {
        let m = self.interval();

        if l % m != 0 {
            self.propagate(l / m);
        }

        if r % m != 0 {
            self.propagate((r - 1) / m);
        }
    }

    fn merge_range(
        &mut self,
        l: usize,
        r: usize,
    ) {
        let m = self.interval();

        if l % m != 0 {
            self.merge(l / m);
        }

        if r % m != 0 {
            self.merge((r - 1) / m);
        }
    }

    pub fn get(
        &mut self,
        i: usize,
    ) -> &O::S {
        self.propagate(i / self.interval());

        &self.data[i]
    }

    pub fn set(
        &mut self,
        i: usize,
        x: O::S,
    ) {
        let j = i / self.interval();

        self.propagate(j);

        self.data[i] = x;

        self.merge(j);
    }

    pub fn fold(
        &mut self,
        l: usize,
        r: usize,
    ) -> O::S {
        assert!(l <= r && r <= self.size());

        let m = self.interval();

        let mut v = self.ops.e();

        self.pull_range(l, r);

        let lj = (l + m - 1) / m;

        let rj = r / m;

        if rj < lj {
            for x in self.data[l..r].iter() {
                v = self.ops.op(v, x.clone());
            }

            return v;
        }

        for x in self.data[l..lj * m].iter() {
            v = self.ops.op(v, x.clone());
        }

        for x in self.buckets[lj..rj].iter() {
            v = self.ops.op(v, x.clone());
        }

        for x in self.data[rj * m..r].iter() {
            v = self.ops.op(v, x.clone());
        }

        v
    }

    pub fn apply(
        &mut self,
        l: usize,
        r: usize,
        f: O::F,
    ) {
        assert!(l <= r && r <= self.size());

        let m = self.interval();

        self.pull_range(l, r);

        let lj = (l + m - 1) / m;

        let rj = r / m;

        if rj < lj {
            for v in self.data[l..r].iter_mut() {
                *v = self.ops.map(f.clone(), v.clone());
            }

            self.merge_range(l, r);

            return;
        }

        for v in self.data[l..lj * m].iter_mut() {
            *v = self.ops.map(f.clone(), v.clone());
        }

        for (v, g) in
            self.buckets[lj..rj].iter_mut().zip(self.lazy[lj..rj].iter_mut())
        {
            *v = self.ops.map(f.clone(), v.clone());

            *g = self.ops.compose(f.clone(), g.clone());
        }

        for v in self.data[rj * m..r].iter_mut() {
            *v = self.ops.map(f.clone(), v.clone());
        }

        self.merge_range(l, r);
    }

    pub fn max_right<F>(
        &mut self,
        is_ok: F,
        l: usize,
    ) -> usize
    where
        F: Fn(&O::S) -> bool,
    {
        let m = self.interval();

        let n = self.size();

        if l % m != 0 {
            self.propagate(l);
        }

        let lj = (l + m - 1) / m;

        let mut v = self.ops.e();

        for i in l..lj * m {
            let nv = self.ops.op(v.clone(), self.data[i].clone());

            if !is_ok(&nv) {
                return i;
            }

            v = nv;
        }

        let mut i = n;

        for j in lj..self.buckets.len() {
            let nv = self.ops.op(v.clone(), self.buckets[j].clone());

            if !is_ok(&nv) {
                i = j * m;

                break;
            }

            v = nv;
        }

        if i != n {
            self.propagate(i / m);
        }

        while i < n {
            let nv = self.ops.op(v.clone(), self.data[i].clone());

            if !is_ok(&nv) {
                return i;
            }

            v = nv;

            i += 1;
        }

        i
    }

    pub fn min_left<F>(
        &mut self,
        is_ok: F,
        r: usize,
    ) -> usize
    where
        F: Fn(&O::S) -> bool,
    {
        let m = self.interval();

        if r % m != 0 {
            self.propagate((r - 1) / m);
        }

        let rj = r / m;

        let mut v = self.ops.e();

        for i in (rj * m..r).rev() {
            let nv = self.ops.op(self.data[i].clone(), v.clone());

            if !is_ok(&nv) {
                return i + 1;
            }

            v = nv;
        }

        let mut i = 0;

        for j in (0..rj).rev() {
            let nv = self.ops.op(self.buckets[j].clone(), v.clone());

            if !is_ok(&nv) {
                i = (j + 1) * m;

                break;
            }

            v = nv;
        }

        if i > 0 {
            self.propagate((i - 1) / m);
        }

        while i > 0 {
            i -= 1;

            let nv = self.ops.op(self.data[i].clone(), v.clone());

            if !is_ok(&nv) {
                return i + 1;
            }

            v = nv;
        }

        i
    }
}

impl<O: Ops> From<(O, &[O::S])> for LazySqrtDecomposition<O>
where
    O::S: Clone + PartialEq,
    O::F: Clone + PartialEq,
{
    fn from(p: (O, &[O::S])) -> Self {
        let (g, data) = p;

        let mut sqd = Self::new(g, data.len());

        sqd.data.clone_from_slice(data);

        for j in 0..sqd.buckets.len() {
            sqd.merge(j);
        }

        sqd
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

        let mut sqd = LazySqrtDecomposition::new(Lz {}, n);

        for i in 0..n {
            sqd.set(i, (0, 1));
        }

        for i in 0..n {
            assert_eq!(sqd.get(i), &(0, 1));
        }

        sqd.apply(1, 3, 1);

        assert_eq!(sqd.fold(0, n), (2, 5));

        assert_eq!(sqd.max_right(|x| x.0 < 2, 0), 2);

        assert_eq!(sqd.max_right(|x| x.0 < 2, 2), n);

        assert_eq!(sqd.min_left(|x| x.0 < 2, n), 2);

        assert_eq!(sqd.min_left(|x| x.0 < 2, 2), 0);
    }
}
