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

pub struct DualSqrtDecomposition<G: Monoid> {
    g: G,
    data: Vec<G::T>,
    buckets: Vec<G::T>,
}

impl<G: Monoid> DualSqrtDecomposition<G>
where
    G::T: Clone + Eq,
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

    fn propagate(
        &mut self,
        j: usize,
    ) {
        let m = self.interval();

        let n = self.size();

        let x = self.buckets[j].clone();

        if x == self.g.e() {
            return;
        }

        for v in self.data[j * m..n.min((j + 1) * m)].iter_mut() {
            *v = self.g.op(v.clone(), x.clone());
        }

        self.buckets[j] = self.g.e();
    }

    pub fn get(
        &mut self,
        i: usize,
    ) -> &mut G::T {
        let m = self.interval();

        self.propagate(i / m);

        &mut self.data[i]
    }

    pub fn operate(
        &mut self,
        l: usize,
        r: usize,
        x: G::T,
    ) {
        assert!(l <= r && r <= self.size());

        let n = self.interval();

        self.propagate(l / n);

        if l < r {
            self.propagate((r - 1) / n);
        }

        let lj = (l + n - 1) / n;

        let rj = r / n;

        if rj < lj {
            for v in self.data[l..r].iter_mut() {
                *v = self.g.op(v.clone(), x.clone());
            }

            return;
        }

        for v in self.data[l..lj * n].iter_mut() {
            *v = self.g.op(v.clone(), x.clone());
        }

        for v in self.buckets[lj..rj].iter_mut() {
            *v = self.g.op(v.clone(), x.clone());
        }

        for v in self.data[rj * n..r].iter_mut() {
            *v = self.g.op(v.clone(), x.clone());
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        struct M;

        impl Monoid for M {
            type T = i64;

            fn op(
                &self,
                l: Self::T,
                r: Self::T,
            ) -> Self::T {
                l + r
            }

            fn e(&self) -> Self::T { 0 }
        }

        let n = 5;

        let mut sqd = DualSqrtDecomposition::new(M {}, n);

        sqd.operate(1, 3, 1);

        assert_eq!(sqd.get(0), &0);

        assert_eq!(sqd.get(1), &1);

        assert_eq!(sqd.get(1), &1);

        assert_eq!(sqd.get(0), &0);

        assert_eq!(sqd.get(0), &0);

        *sqd.get(0) = -1;

        sqd.operate(0, 2, -1);

        assert_eq!(sqd.get(0), &-2);

        assert_eq!(sqd.get(1), &0);

        assert_eq!(sqd.get(2), &1);
    }
}
