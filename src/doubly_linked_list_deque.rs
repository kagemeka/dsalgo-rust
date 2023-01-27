use crate::doubly_linked_list_node::{
    Cell,
    Node,
};

pub struct Deque<T> {
    left: Option<Cell<T>>,
    right: Option<Cell<T>>,
    size: usize,
}

impl<T: Clone> Deque<T> {
    pub fn new() -> Self { Self { left: None, right: None, size: 0 } }

    pub fn size(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    pub fn push_right(
        &mut self,
        x: T,
    ) {
        let x = Node::new(x);

        let right = Node::add_right(self.right.take(), x);

        if self.size == 0 {
            self.left = Some(right.clone());
        }

        self.right = Some(right);

        self.size += 1;
    }

    pub fn push_left(
        &mut self,
        x: T,
    ) {
        let x = Node::new(x);

        let left = Node::add_left(self.left.take(), x);

        if self.size == 0 {
            self.right = Some(left.clone());
        }

        self.left = Some(left);

        self.size += 1;
    }

    pub fn pop_right(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let (right, popped) = Node::split_left(self.right.take().unwrap());

        let v = popped.borrow().value.clone();

        self.size -= 1;

        if self.size == 0 {
            self.left = None;
        }

        self.right = right;

        Some(v)
    }

    pub fn pop_left(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let (popped, left) = Node::split_right(self.left.take().unwrap());

        let v = popped.borrow().value.clone();

        self.size -= 1;

        if self.size == 0 {
            self.right = None;
        }

        self.left = left;

        Some(v)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = Deque::new();

        que.push_right(0);

        que.push_left(1);

        que.push_right(2);

        assert_eq!(que.size(), 3);

        assert_eq!(que.pop_left(), Some(1));

        assert_eq!(que.pop_left(), Some(0));

        assert_eq!(que.pop_right(), Some(2));

        assert_eq!(que.pop_right(), None);
    }
}
