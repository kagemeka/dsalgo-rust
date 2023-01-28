use crate::avl_tree_node_with_value_size_box_recurse::Node;

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

        self.0 = Some(Node::insert(self.0.take(), i, Node::new(value)));
    }

    pub fn remove(
        &mut self,
        value: &T,
    ) {
        if !self.contains(value) {
            return;
        }

        let i = self.lower_bound(value);

        self.0 = Node::remove(self.0.take().unwrap(), i);
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

        s.remove(&"b");

        for &v in s.iter() {
            println!("{:?}", v);
        }
    }

    #[test]

    fn test_arc033_3() {
        let cases = vec![
            vec![
                ((1, 11), 0),
                ((1, 29), 0),
                ((1, 89), 0),
                ((2, 2), 29),
                ((2, 2), 89),
            ],
            vec![
                ((1, 8932), 0),
                ((1, 183450), 0),
                ((1, 34323), 0),
                ((1, 81486), 0),
                ((1, 127874), 0),
                ((1, 114850), 0),
                ((1, 55277), 0),
                ((1, 112706), 0),
                ((2, 3), 55277),
                ((1, 39456), 0),
                ((1, 52403), 0),
                ((2, 4), 52403),
            ],
        ];

        for q in cases {
            let mut s = AVLMultiset::new();

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let v = s[x - 1];

                    assert_eq!(v, ans);

                    s.remove(&v);
                }
            }
        }
    }
}
