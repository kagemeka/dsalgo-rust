pub trait Monoid {
    type T;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;
}

pub struct Fenwick<G: Monoid> {
    pub(crate) g: G,
    pub(crate) node: Vec<G::T>,
}

impl<G: Monoid> Fenwick<G>
where
    G::T: Clone,
{
    pub fn new(
        g: G,
        size: usize,
    ) -> Self {
        let node = vec![g.e(); size + 1];

        Self { g, node }
    }

    pub fn size(&self) -> usize { self.node.len() - 1 }

    pub fn operate(
        &mut self,
        mut i: usize,
        x: G::T,
    ) {
        let n = self.size();

        assert!(i < n);

        i += 1;

        while i <= n {
            self.node[i] = self.g.op(self.node[i].clone(), x.clone());

            i += 1 << i.trailing_zeros();
        }
    }

    pub fn fold_lt(
        &self,
        mut i: usize,
    ) -> G::T {
        assert!(i <= self.size());

        let mut v = self.g.e();

        while i > 0 {
            v = self.g.op(v, self.node[i].clone());

            i -= 1 << i.trailing_zeros();
        }

        v
    }

    pub fn max_right<F: Fn(&G::T) -> bool>(
        &self,
        is_ok: F,
    ) -> usize {
        let n = self.size();

        let mut v = self.g.e();

        let mut i = 0;

        let mut d = (n + 1).next_power_of_two();

        loop {
            d >>= 1;

            if d == 0 {
                return i;
            }

            if i + d > n {
                continue;
            }

            let nv = self.g.op(v.clone(), self.node[i + d].clone());

            if is_ok(&nv) {
                v = nv;

                i += d;
            }
        }
    }
}

impl<G: Monoid> From<(G, &[G::T])> for Fenwick<G>
where
    G::T: Clone,
{
    fn from(p: (G, &[G::T])) -> Self {
        let (g, data) = p;

        let n = data.len();

        let mut fw = Self::new(g, n);

        fw.node[1..].clone_from_slice(data);

        for i in 1..n {
            let j = i + (1 << i.trailing_zeros());

            if j <= n {
                fw.node[j] = fw.g.op(fw.node[j].clone(), fw.node[i].clone());
            }
        }

        fw
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

        let a = vec![0, 1, 0, 0];

        let n = a.len();

        let mut fw = Fenwick::from((G {}, a.as_slice()));

        assert_eq!(fw.size(), n);

        assert_eq!(fw.fold_lt(n), 1);

        fw.operate(2, 1);

        assert_eq!(fw.fold_lt(2), 1);

        assert_eq!(fw.fold_lt(3), 2);

        assert_eq!(fw.max_right(|v| v <= &-1), 0);

        assert_eq!(fw.max_right(|v| v <= &0), 1);

        assert_eq!(fw.max_right(|v| v <= &1), 2);

        assert_eq!(fw.max_right(|v| v <= &2), n);
    }
}
