use crate::avl_tree_node_with_rc_refcell_merge_split_based_recurse::*;

#[derive(Debug)]

pub struct AVLMultiset<T>(Option<Cell<T>>);

impl<T: Ord + Clone> AVLMultiset<T> {
    pub fn new() -> Self { Self(None) }

    pub fn size(&self) -> usize { Node::size(self.0.as_ref()) }

    pub fn lower_bound(
        &self,
        value: &T,
    ) -> usize {
        Node::binary_search(|v| v >= &value, self.0.as_ref())
    }

    pub fn upper_bound(
        &self,
        value: &T,
    ) -> usize {
        Node::binary_search(|v| v > &value, self.0.as_ref())
    }

    pub fn count(
        &self,
        value: &T,
    ) -> usize {
        self.upper_bound(value) - self.lower_bound(value)
    }

    pub fn contains(
        &self,
        value: &T,
    ) -> bool {
        self.count(value) > 0
    }

    pub fn insert(
        &mut self,
        value: T,
    ) {
        let i = self.lower_bound(&value);

        self.0 = Node::insert(self.0.take(), i, Some(Node::new(value)));
    }

    pub fn remove(
        &mut self,
        value: &T,
    ) {
        if !self.contains(value) {
            return;
        }

        let i = self.lower_bound(value);

        self.0 = Node::remove(self.0.take(), i);
    }

    pub fn remove_all(
        &mut self,
        value: &T,
    ) {
        let l = self.lower_bound(value);

        let r = self.upper_bound(value);

        self.0 = Node::remove_range(self.0.take(), l, r);
    }

    pub fn get(
        &mut self,
        i: usize,
    ) -> T {
        let (kth_node, root) = Node::kth_node(self.0.take().unwrap(), i);

        self.0 = Some(root);

        let v = kth_node.borrow().value.clone();

        v
    }
}
