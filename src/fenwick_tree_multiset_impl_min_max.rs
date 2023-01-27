use crate::fenwick_tree_multiset::*;

impl Multiset {
    pub fn min(&self) -> Option<usize> { self.get(0) }

    pub fn max(&self) -> Option<usize> { self.get(self.size() - 1) }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
