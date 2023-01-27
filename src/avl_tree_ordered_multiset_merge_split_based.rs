use crate::avl_tree_node_with_box_recurse_merge_split_based::Node;

#[derive(Debug)]

pub struct AVLMultiset<T>(Option<Box<Node<T>>>);

impl<T: Ord> AVLMultiset<T> {
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

    pub fn iter<'a>(&'a self) -> std::vec::IntoIter<&'a T> {
        self.0.as_ref().unwrap().iter()
    }
}

use std::ops::*;

impl<T> Index<usize> for AVLMultiset<T> {
    type Output = T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &Node::kth_node(self.0.as_ref().unwrap(), i).value
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = AVLMultiset::new();

        s.insert("b");

        s.insert("a");

        s.insert("b");

        println!("{:?}", s);

        println!("{:?}", s[0]);

        println!("{:?}", s[1]);

        println!("{:?}", s[2]);

        assert_eq!(s.count(&"b"), 2);

        s.remove_all(&"b");

        for &v in s.iter() {
            println!("{:?}", v);
        }
    }
}
