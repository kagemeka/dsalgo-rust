//! cumulative sum of cumulative sum of cumulative sum.

use crate::fenwick_tree_dual_i64_add_1_indexed::Fenwick;

pub struct CumulativeSum3(Fenwick, Fenwick, Fenwick);

impl CumulativeSum3 {
    pub fn new(size: usize) -> Self {
        Self(Fenwick::new(size), Fenwick::new(size), Fenwick::new(size))
    }

    pub fn size(&self) -> usize { self.0.size() }

    pub fn add(
        &mut self,
        i: usize,
        x: i64,
    ) {
        let j = i as i64;

        self.0.add_ge(i, (1 - j) * (2 - j) * x);

        self.1.add_ge(i, (3 - 2 * j) * x);

        self.2.add_ge(i, x);
    }

    pub fn get(
        &self,
        i: usize,
    ) -> i64 {
        let j = i as i64;

        (self.0.get(i) + self.1.get(i) * j + self.2.get(i) * j * j) / 2
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = CumulativeSum3::new(10);

        s.add(0, 1);

        s.add(1, 2);

        s.add(2, 3);

        assert_eq!(s.get(2), 15);

        s.add(1, -2);

        assert_eq!(s.get(2), 9);
    }
}
