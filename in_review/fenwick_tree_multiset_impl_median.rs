use crate::fenwick_tree_multiset::*;

impl Multiset {
    pub fn low_median(&self) -> Option<usize> {
        self.get((self.size() - 1) >> 1)
    }

    pub fn high_median(&self) -> Option<usize> { self.get(self.size() >> 1) }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
