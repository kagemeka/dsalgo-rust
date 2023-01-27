pub struct Segtree {
    pub size: usize,
    pub(crate) node: Vec<i64>,
}

impl Segtree {
    fn n(&self) -> usize { self.node.len() >> 1 }

    fn merge(
        &mut self,
        i: usize,
    ) {
        self.node[i] = self.node[i << 1] + self.node[i << 1 | 1];
    }

    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let node = vec![0; size.next_power_of_two() << 1];

        Self { size, node }
    }

    pub fn set(
        &mut self,
        mut i: usize,
        x: i64,
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
    ) -> i64 {
        assert!(l <= r && r <= self.size);

        let mut vl = 0;

        let mut vr = 0;

        let n = self.n();

        l += n;

        r += n;

        while l < r {
            if l & 1 == 1 {
                vl += self.node[l];

                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;

                vr = self.node[r] + vr;
            }

            l >>= 1;

            r >>= 1;
        }

        vl + vr
    }

    pub fn max_right<F: Fn(&i64) -> bool>(
        &self,
        f: F,
        l: usize,
    ) -> usize {
        assert!(l <= self.size);

        if l == self.size {
            return self.size;
        }

        let mut v = 0;

        let n = self.n();

        let mut i = l + n;

        loop {
            i >>= i.trailing_zeros();

            let nv = v + self.node[i];

            if !f(&nv) {
                break;
            }

            v = nv;

            i += 1;

            if i.count_ones() == 1 {
                return self.size;
            }
        }

        while i < n {
            i <<= 1;

            let nv = v + self.node[i];

            if !f(&nv) {
                continue;
            }

            v = nv;

            i += 1;
        }

        i - n
    }
}

use std::ops::*;

impl Index<usize> for Segtree {
    type Output = i64;

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
