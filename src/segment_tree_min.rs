pub struct Segtree<T> {
    pub size: usize,
    inf: T,
    pub(crate) node: Vec<T>,
}

impl<T> Segtree<T> {
    fn n(&self) -> usize { self.node.len() >> 1 }
}

impl<T: Ord + Clone> Segtree<T> {
    fn update(
        &mut self,
        i: usize,
    ) {
        self.node[i] =
            self.node[i << 1].clone().min(self.node[i << 1 | 1].clone());
    }

    pub fn new(
        inf: T,
        size: usize,
    ) -> Self {
        assert!(size > 0);

        let node = vec![inf.clone(); size.next_power_of_two() << 1];

        Self { inf, size, node }
    }

    pub fn set(
        &mut self,
        mut i: usize,
        x: T,
    ) {
        assert!(i < self.size);

        i += self.n();

        self.node[i] = x;

        while i > 1 {
            i >>= 1;

            self.update(i);
        }
    }

    pub fn fold(
        &self,
        mut l: usize,
        mut r: usize,
    ) -> T {
        assert!(l <= r && r <= self.size);

        let mut vl = self.inf.clone();

        let mut vr = self.inf.clone();

        let n = self.n();

        l += n;

        r += n;

        while l < r {
            if l & 1 == 1 {
                vl = vl.min(self.node[l].clone());

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                vr = self.node[r].clone().min(vr);
            }

            l >>= 1;

            r >>= 1;
        }

        vl.min(vr)
    }
}

use std::ops::*;

impl<T> Index<usize> for Segtree<T> {
    type Output = T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.node[i + self.n()]
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
