use std::ops::*;

use crate::bit_length_with_count_leading_zeros_usize::*;

pub struct DualSegtree<T> {
    node: Vec<T>,
    size: usize,
}

impl<T: Clone + Add<Output = T> + From<i32>> DualSegtree<T> {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let n = size.next_power_of_two();

        let node = vec![0.into(); n << 1];

        Self { node, size }
    }

    pub fn size(&self) -> usize { self.size }

    fn n(&self) -> usize { self.node.len() >> 1 }

    fn height(&self) -> usize { bit_length(self.n()) }

    fn operate_node(
        &mut self,
        i: usize,
        x: T,
    ) {
        self.node[i] = self.node[i].clone() + x;
    }

    fn propagate(
        &mut self,
        i: usize,
    ) {
        self.operate_node(i << 1, self.node[i].clone());

        self.operate_node(i << 1 | 1, self.node[i].clone());

        self.node[i] = 0.into();
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
    ) -> &mut T {
        assert!(i < self.size());

        i += self.n();

        self.pull(i);

        &mut self.node[i]
    }

    pub fn operate(
        &mut self,
        mut l: usize,
        mut r: usize,
        x: T,
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
        let n = 5;

        let mut seg = DualSegtree::<i64>::new(n);

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
