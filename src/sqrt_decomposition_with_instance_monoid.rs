use crate::integer_square_root_with_binary_search_usize::isqrt;

pub trait Monoid {
    type T;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;
}

pub struct SqrtDecomposition<G: Monoid> {
    g: G,
    data: Vec<G::T>,
    buckets: Vec<G::T>,
}

impl<G: Monoid> SqrtDecomposition<G>
where
    G::T: Clone,
{
    pub fn size(&self) -> usize { self.data.len() }

    pub fn interval(&self) -> usize {
        let n = self.buckets.len();

        (self.size() + n - 1) / n
    }

    pub fn new(
        g: G,
        size: usize,
    ) -> Self {
        let data = vec![g.e(); size];

        let m = isqrt(size);

        let buckets = vec![g.e(); (size + m - 1) / m];

        Self { g, data, buckets }
    }

    fn merge(
        &mut self,
        j: usize,
    ) {
        let n = self.interval();

        self.buckets[j] = self.data[j * n..self.size().min((j + 1) * n)]
            .iter()
            .cloned()
            .fold(self.g.e(), |x, y| self.g.op(x, y))
        // .reduce(|l, r| self.g.op(l, r))
        // .unwrap();
    }

    pub fn set(
        &mut self,
        i: usize,
        x: G::T,
    ) {
        self.data[i] = x;

        self.merge(i / self.interval());
    }

    pub fn fold(
        &self,
        l: usize,
        r: usize,
    ) -> G::T {
        assert!(l <= r && r <= self.size());

        let n = self.interval();

        let mut v = self.g.e();

        let lj = (l + n - 1) / n;

        let rj = r / n;

        if rj < lj {
            for x in self.data[l..r].iter() {
                v = self.g.op(v, x.clone());
            }

            return v;
        }

        for x in self.data[l..lj * n].iter() {
            v = self.g.op(v, x.clone());
        }

        for x in self.buckets[lj..rj].iter() {
            v = self.g.op(v, x.clone());
        }

        for x in self.data[rj * n..r].iter() {
            v = self.g.op(v, x.clone());
        }

        v
    }

    pub fn max_right<F>(
        &self,
        is_ok: F,
        l: usize,
    ) -> usize
    where
        F: Fn(&G::T) -> bool,
    {
        let m = self.interval();

        let n = self.size();

        let lj = (l + m - 1) / m;

        let mut v = self.g.e();

        for i in l..lj * m {
            let nv = self.g.op(v.clone(), self.data[i].clone());

            if !is_ok(&nv) {
                return i;
            }

            v = nv;
        }

        let mut i = n;

        for j in lj..self.buckets.len() {
            let nv = self.g.op(v.clone(), self.buckets[j].clone());

            if !is_ok(&nv) {
                i = j * m;

                break;
            }

            v = nv;
        }

        while i < n {
            let nv = self.g.op(v.clone(), self.data[i].clone());

            if !is_ok(&nv) {
                return i;
            }

            v = nv;

            i += 1;
        }

        i
    }

    pub fn min_left<F>(
        &self,
        is_ok: F,
        r: usize,
    ) -> usize
    where
        F: Fn(&G::T) -> bool,
    {
        let m = self.interval();

        let rj = r / m;

        let mut v = self.g.e();

        for i in (rj * m..r).rev() {
            let nv = self.g.op(self.data[i].clone(), v.clone());

            if !is_ok(&nv) {
                return i + 1;
            }

            v = nv;
        }

        let mut i = 0;

        for j in (0..rj).rev() {
            let nv = self.g.op(self.buckets[j].clone(), v.clone());

            if !is_ok(&nv) {
                i = (j + 1) * m;

                break;
            }

            v = nv;
        }

        while i > 0 {
            i -= 1;

            let nv = self.g.op(self.data[i].clone(), v.clone());

            if !is_ok(&nv) {
                return i + 1;
            }

            v = nv;
        }

        i
    }
}

use std::ops::*;

impl<G: Monoid> Index<usize> for SqrtDecomposition<G> {
    type Output = G::T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.data[i]
    }
}

impl<G: Monoid> From<(G, &[G::T])> for SqrtDecomposition<G>
where
    G::T: Clone,
{
    fn from(p: (G, &[G::T])) -> Self {
        let (g, data) = p;

        let mut sqd = SqrtDecomposition::new(g, data.len());

        sqd.data.clone_from_slice(data);

        for j in 0..sqd.buckets.len() {
            sqd.merge(j);
        }

        sqd
    }
}

#[cfg(test)]

mod tests {

    struct G;

    impl Monoid for G {
        type T = i32;

        fn op(
            &self,
            x: i32,
            y: i32,
        ) -> i32 {
            x + y
        }

        fn e(&self) -> i32 { 0 }
    }

    use super::*;

    #[test]

    fn test() {
        let mut sqd = SqrtDecomposition::new(G {}, 5);

        for i in 0..5 {
            sqd.set(i, i as i32 + 1);
        }

        assert_eq!(sqd[3], 4);

        assert_eq!(sqd.fold(2, 4), 7);

        assert_eq!(sqd.max_right(|&x| x < 5, 1), 2);

        assert_eq!(sqd.max_right(|&x| x <= 5, 1), 3);

        assert_eq!(sqd.min_left(|&x| x < 7, 4), 3);

        assert_eq!(sqd.min_left(|&x| x <= 7, 4), 2);
    }
}
