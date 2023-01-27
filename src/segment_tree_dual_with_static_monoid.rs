use crate::bit_length_with_count_leading_zeros_usize::bit_length;

pub trait Monoid {
    type T;

    fn op(
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e() -> Self::T;
}

pub struct DualSegtree<G: Monoid> {
    node: Vec<G::T>,
    size: usize,
}

impl<G: Monoid> DualSegtree<G>
where
    G::T: Clone,
{
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let n = size.next_power_of_two();

        let node = vec![G::e(); n << 1];

        Self { node, size }
    }

    pub fn size(&self) -> usize { self.size }

    fn n(&self) -> usize { self.node.len() >> 1 }

    fn height(&self) -> usize { bit_length(self.n()) }

    fn operate_node(
        &mut self,
        i: usize,
        x: G::T,
    ) {
        self.node[i] = G::op(self.node[i].clone(), x);
    }

    fn propagate(
        &mut self,
        i: usize,
    ) {
        self.operate_node(i << 1, self.node[i].clone());

        self.operate_node(i << 1 | 1, self.node[i].clone());

        self.node[i] = G::e();
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
