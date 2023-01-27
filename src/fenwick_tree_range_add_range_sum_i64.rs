use crate::fenwick_tree_dynamic_cumulative_sum_2_i64::CumulativeSum2;

pub struct RangeAddRangeSum(CumulativeSum2);

impl RangeAddRangeSum {
    pub fn new(size: usize) -> Self { Self(CumulativeSum2::new(size)) }

    pub fn size(&self) -> usize { self.0.size() }

    pub fn add(
        &mut self,
        l: usize,
        r: usize,
        x: i64,
    ) {
        assert!(l < r && r <= self.size());

        self.0.add(l, x);

        if r < self.size() {
            self.0.add(r, -x);
        }
    }

    pub fn sum(
        &self,
        l: usize,
        r: usize,
    ) -> i64 {
        assert!(l < r && r <= self.size());

        let mut s = self.0.get(r - 1);

        if l > 0 {
            s -= self.0.get(l - 1);
        }

        s
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = RangeAddRangeSum::new(10);

        s.add(2, 5, 3);

        assert_eq!(s.sum(0, 10), 9);

        assert_eq!(s.sum(2, 4), 6);
    }
}
