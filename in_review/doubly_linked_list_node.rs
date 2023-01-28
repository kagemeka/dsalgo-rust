use std::{
    cell::RefCell,
    rc::Rc,
};

pub type Cell<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    left: Option<Cell<T>>,
    right: Option<Cell<T>>,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Cell<T> {
        Rc::new(RefCell::new(Self { left: None, right: None, value }))
    }

    pub fn add_right(
        node: Option<Cell<T>>,
        new: Cell<T>,
    ) -> Cell<T> {
        if let Some(node) = node {
            node.borrow_mut().right = Some(new.clone());

            new.borrow_mut().left = Some(node.clone());
        }

        new
    }

    pub fn add_left(
        node: Option<Cell<T>>,
        new: Cell<T>,
    ) -> Cell<T> {
        if let Some(node) = node {
            node.borrow_mut().left = Some(new.clone());

            new.borrow_mut().right = Some(node.clone());
        }

        new
    }

    pub fn split_right(node: Cell<T>) -> (Cell<T>, Option<Cell<T>>) {
        let right = node.borrow_mut().right.take();

        if let Some(right) = right.as_ref() {
            right.borrow_mut().left = None;
        }

        (node, right)
    }

    pub fn split_left(node: Cell<T>) -> (Option<Cell<T>>, Cell<T>) {
        let left = node.borrow_mut().left.take();

        if let Some(left) = left.as_ref() {
            left.borrow_mut().right = None;
        }

        (left, node)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // let mut
    }
}
