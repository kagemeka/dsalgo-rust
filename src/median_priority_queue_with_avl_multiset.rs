use crate::avl_tree_ordered_multiset::AVLMultiset;

impl<T: Ord> AVLMultiset<T> {
    pub fn low_median(&self) -> &T { &self[(self.size() - 1) >> 1] }

    pub fn high_median(&self) -> &T { &self[self.size() >> 1] }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
