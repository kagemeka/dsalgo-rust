use std::ops::*;

pub struct Segtree<T> {
    pub size: usize,
    zero: T,
    pub(crate) node: Vec<T>,
}

impl<T> Segtree<T> {
    fn n(&self) -> usize { self.node.len() >> 1 }
}

impl<T> Index<usize> for Segtree<T> {
    type Output = T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.node[i + self.n()]
    }
}

impl<T: Add<Output = T> + Clone> Segtree<T> {
    fn merge(
        &mut self,
        i: usize,
    ) {
        self.node[i] =
            self.node[i << 1].clone() + self.node[i << 1 | 1].clone();
    }

    pub fn new(
        zero: T,
        size: usize,
    ) -> Self {
        assert!(size > 0);

        let node = vec![zero.clone(); size.next_power_of_two() << 1];

        Self { zero, size, node }
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

            self.merge(i);
        }
    }

    pub fn fold(
        &self,
        mut l: usize,
        mut r: usize,
    ) -> T {
        assert!(l <= r && r <= self.size);

        let mut vl = self.zero.clone();

        let mut vr = self.zero.clone();

        let n = self.n();

        l += n;

        r += n;

        while l < r {
            if l & 1 == 1 {
                vl = vl + self.node[l].clone();

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                vr = self.node[r].clone() + vr;
            }

            l >>= 1;

            r >>= 1;
        }

        vl + vr
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 5;

        let mut seg = Segtree::<i64>::new(0, n);

        seg.set(2, 1);

        assert_eq!(seg[2], 1);

        assert_eq!(seg.fold(0, 5), 1);
    }
}
