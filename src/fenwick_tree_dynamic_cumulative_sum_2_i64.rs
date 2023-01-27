//! cumulative sum of cumulative sum.

use crate::fenwick_tree_dual_i64_add_1_indexed::Fenwick;

pub struct CumulativeSum2(Fenwick, Fenwick);

impl CumulativeSum2 {
    pub fn new(size: usize) -> Self {
        Self(Fenwick::new(size), Fenwick::new(size))
    }

    pub fn size(&self) -> usize { self.0.size() }

    pub fn add(
        &mut self,
        i: usize,
        x: i64,
    ) {
        self.0.add_ge(i, (1 - i as i64) * x);

        self.1.add_ge(i, x);
    }

    pub fn get(
        &self,
        i: usize,
    ) -> i64 {
        self.0.get(i) + self.1.get(i) * i as i64
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = CumulativeSum2::new(10);

        s.add(0, 1);

        assert_eq!(s.get(9), 10);

        for i in 1..10 {
            s.add(i, 1);
        }

        assert_eq!(s.get(9), 55);
    }
}
