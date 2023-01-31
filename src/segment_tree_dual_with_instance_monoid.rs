use crate::bit_length_with_count_leading_zeros_usize::bit_length;

pub trait Monoid {
    type T;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;
}

pub struct DualSegtree<G: Monoid> {
    g: G,
    node: Vec<G::T>,
    size: usize,
}

impl<G: Monoid> DualSegtree<G>
where
    G::T: Clone,
{
    pub fn new(
        g: G,
        size: usize,
    ) -> Self {
        assert!(size > 0);

        let n = size.next_power_of_two();

        let node = vec![g.e(); n << 1];

        Self { g, node, size }
    }

    pub fn size(&self) -> usize { self.size }

    fn n(&self) -> usize { self.node.len() >> 1 }

    fn height(&self) -> usize { bit_length(self.n()) }

    fn operate_node(
        &mut self,
        i: usize,
        x: G::T,
    ) {
        self.node[i] = self.g.op(self.node[i].clone(), x);
    }

    fn propagate(
        &mut self,
        i: usize,
    ) {
        self.operate_node(i << 1, self.node[i].clone());

        self.operate_node(i << 1 | 1, self.node[i].clone());

        self.node[i] = self.g.e();
    }

    fn pull(
        &mut self,
        i: usize,
    ) {
        for j in (1..self.height()).rev() {
            self.propagate(i >> j);
        }
    }

    pub fn get(
        &mut self,
        mut i: usize,
    ) -> &mut G::T {
        assert!(i < self.size());

        i += self.n();

        self.pull(i);

        &mut self.node[i]
    }

    pub fn operate(
        &mut self,
        mut l: usize,
        mut r: usize,
        x: G::T,
    ) {
        assert!(l <= r && r <= self.size());

        let n = self.n();

        l += n;

        r += n;

        self.pull(l >> l.trailing_zeros());

        self.pull((r >> r.trailing_zeros()) - 1);

        while l < r {
            if l & 1 == 1 {
                self.operate_node(l, x.clone());

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                self.operate_node(r, x.clone());
            }

            l >>= 1;

            r >>= 1;
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

        let mut seg = DualSegtree::new(M {}, n);

        seg.operate(1, 3, 1);

        assert_eq!(seg.get(0), &0);

        assert_eq!(seg.get(1), &1);

        assert_eq!(seg.get(1), &1);

        assert_eq!(seg.get(0), &0);

        assert_eq!(seg.get(0), &0);

        *seg.get(0) = -1;

        seg.operate(0, 2, -1);

        assert_eq!(seg.get(0), &-2);

        assert_eq!(seg.get(1), &0);

        assert_eq!(seg.get(2), &1);
    }
}
