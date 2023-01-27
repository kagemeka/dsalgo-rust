use std::{
    cell::RefCell,
    rc::Rc,
};

pub type Cell<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]

pub struct Node<T> {
    pub left: Option<Cell<T>>,
    pub right: Option<Cell<T>>,
    pub height: usize,
    pub size: usize,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Cell<T> {
        Rc::new(RefCell::new(Self {
            left: None,
            right: None,
            height: 1,
            size: 1,
            value,
        }))
    }

    pub(crate) fn height(node: Option<&Cell<T>>) -> usize {
        if let Some(node) = node {
            node.borrow().height
        } else {
            0
        }
    }

    pub(crate) fn size(node: Option<&Cell<T>>) -> usize {
        if let Some(node) = node {
            node.borrow().size
        } else {
            0
        }
    }

    pub(crate) fn balance(node: Option<&Cell<T>>) -> isize {
        if let Some(node) = node {
            Self::height(node.borrow().right.as_ref()) as isize
                - Self::height(node.borrow().left.as_ref()) as isize
        } else {
            0
        }
    }

    pub(crate) fn update(&mut self) {
        let hl = Self::height(self.left.as_ref());

        let hr = Self::height(self.right.as_ref());

        self.height = hl.max(hr) + 1;

        let sl = Self::size(self.left.as_ref());

        let sr = Self::size(self.right.as_ref());

        self.size = sl + sr + 1;
    }

    pub(crate) fn rotate_left(root: Cell<T>) -> Cell<T> {
        let new_root = root.borrow_mut().right.take().unwrap();

        root.borrow_mut().right = new_root.borrow_mut().left.take();

        root.borrow_mut().update();

        new_root.borrow_mut().left = Some(root);

        new_root.borrow_mut().update();

        new_root
    }

    pub(crate) fn rotate_right(root: Cell<T>) -> Cell<T> {
        let new_root = root.borrow_mut().left.take().unwrap();

        root.borrow_mut().left = new_root.borrow_mut().right.take();

        root.borrow_mut().update();

        new_root.borrow_mut().right = Some(root);

        new_root.borrow_mut().update();

        new_root
    }

    pub(crate) fn rebalance(root: Cell<T>) -> Cell<T> {
        root.borrow_mut().update();

        let b = Self::balance(Some(&root));

        if b < -1 {
            if Self::balance(root.borrow().left.as_ref()) > 0 {
                let left = Some(Self::rotate_left(
                    root.borrow_mut().left.take().unwrap(),
                ));

                root.borrow_mut().left = left;
            }

            return Self::rotate_right(root);
        } else if b > 1 {
            if Self::balance(root.borrow().right.as_ref()) < 0 {
                let right = Some(Self::rotate_right(
                    root.borrow_mut().right.take().unwrap(),
                ));

                root.borrow_mut().right = right;
            }

            return Self::rotate_left(root);
        }

        root
    }

    pub(crate) fn pop_last(root: Cell<T>) -> (Cell<T>, Option<Cell<T>>) {
        if root.borrow().right.is_none() {
            let new_root = root.borrow_mut().left.take();

            return (root, new_root);
        }

        let (last_node, new_right) =
            Self::pop_last(root.borrow_mut().right.take().unwrap());

        root.borrow_mut().right = new_right;

        (last_node, Some(Self::rebalance(root)))
    }

    pub fn merge(
        left: Option<Cell<T>>,
        right: Option<Cell<T>>,
    ) -> Option<Cell<T>> {
        if left.is_none() {
            return right;
        }

        let (root, left) = Self::pop_last(left.unwrap());

        root.borrow_mut().left = left;

        root.borrow_mut().right = right;

        Some(Self::rebalance(root))
    }

    pub fn split(
        root: Option<Cell<T>>,
        i: usize,
    ) -> (Option<Cell<T>>, Option<Cell<T>>) {
        assert!(i <= Self::size(root.as_ref()));

        if root.is_none() {
            return (None, None);
        }

        let root = root.unwrap();

        let lsize = Self::size(root.borrow().left.as_ref());

        if i == lsize {
            let left = root.borrow_mut().left.take();

            (left, Some(Self::rebalance(root)))
        } else if i < lsize {
            let (left, right) = Self::split(root.borrow_mut().left.take(), i);

            root.borrow_mut().left = right;

            (left, Some(Self::rebalance(root)))
        } else {
            let (left, right) =
                Self::split(root.borrow_mut().right.take(), i - lsize - 1);

            root.borrow_mut().right = left;

            (Some(Self::rebalance(root)), right)
        }
    }

    pub fn insert(
        root: Option<Cell<T>>,
        i: usize,
        node: Option<Cell<T>>,
    ) -> Option<Cell<T>> {
        let (left, right) = Self::split(root, i);

        Self::merge(Self::merge(left, node), right)
    }

    pub fn remove_range(
        root: Option<Cell<T>>,
        l: usize,
        r: usize,
    ) -> Option<Cell<T>> {
        let (left, right) = Self::split(root, l);

        let (_, right) = Self::split(right, r - l);

        Self::merge(left, right)
    }

    pub fn remove(
        root: Option<Cell<T>>,
        i: usize,
    ) -> Option<Cell<T>> {
        Self::remove_range(root, i, i + 1)
    }

    pub fn kth_node(
        root: Cell<T>,
        k: usize,
    ) -> (Cell<T>, Cell<T>) {
        assert!(k < root.borrow().size);

        let lsize = Self::size(root.borrow().left.as_ref());

        let ret: Cell<T>;

        if k < lsize {
            let (res, left) =
                Self::kth_node(root.borrow_mut().left.take().unwrap(), k);

            root.borrow_mut().left = Some(left);

            ret = res;
        } else if k > lsize {
            let (res, right) = Self::kth_node(
                root.borrow_mut().right.take().unwrap(),
                k - lsize - 1,
            );

            root.borrow_mut().right = Some(right);

            ret = res;
        } else {
            ret = root.clone();
        }

        (ret, Self::rebalance(root))
    }

    pub fn binary_search<F>(
        f: F,
        root: Option<&Cell<T>>,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        if f(&root.borrow().value) {
            Self::binary_search(f, root.borrow().left.as_ref())
        } else {
            let offset = Self::size(root.borrow().left.as_ref()) + 1;

            offset + Self::binary_search(f, root.borrow().right.as_ref())
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Nd = Node<usize>;

        let mut root = Some(Nd::new(1));

        println!("{:?}", root);

        root = Nd::insert(root, 1, Some(Nd::new(2)));

        println!("{:?}", root);

        root = Nd::insert(root, 2, Some(Nd::new(3)));

        println!("{:?}", root);

        root = Nd::remove(root, 1);

        println!("{:?}", root);

        root = Nd::remove(root, 0);

        println!("{:?}", root);

        assert_eq!(Nd::kth_node(root.clone().unwrap(), 0).0.borrow().value, 3);

        root = Nd::insert(root, 1, Some(Nd::new(1)));

        println!("{:?}", root);

        assert_eq!(Nd::binary_search(|v| v <= &1, root.as_ref()), 1);

        assert_eq!(Nd::binary_search(|v| v < &1, root.as_ref()), 2);

        assert!(!std::ptr::eq(&Nd::new(1), &Nd::new(1)));
    }
}
