use std::{
    cell::RefCell,
    rc::Rc,
};

pub struct Node<T> {
    p: ON<T>,
    l: ON<T>,
    r: ON<T>,
    size: usize,
    pub v: T,
}

use std::ptr;

type Cell<T> = RefCell<Node<T>>;

type N<T> = Rc<Cell<T>>;

pub(crate) type ON<T> = Option<N<T>>;

type ORN<'a, T> = Option<&'a N<T>>;

impl<T> Node<T> {
    pub fn new(v: T) -> N<T> {
        Rc::new(RefCell::new(Self { p: None, l: None, r: None, size: 1, v }))
    }

    pub(crate) fn size(root: ORN<T>) -> usize {
        root.map_or(0, |root| root.borrow().size)
    }

    fn lsize(&self) -> usize { self.l.as_ref().map_or(0, |l| l.borrow().size) }

    fn rsize(&self) -> usize { self.r.as_ref().map_or(0, |r| r.borrow().size) }

    fn update(&mut self) { self.size = self.lsize() + self.rsize() + 1; }

    fn state(&self) -> isize {
        if let Some(p) = self.p.as_ref() {
            if p.borrow().l.is_some()
                && ptr::eq(p.borrow().l.as_ref().unwrap().as_ptr(), self)
            {
                -1
            } else {
                1
            }
        } else {
            0
        }
    }

    fn rotate(node: &N<T>) {
        let s = node.borrow().state();

        assert!(s == -1 || s == 1);

        let p = node.borrow_mut().p.take().unwrap();

        let ps = p.borrow().state();

        let pp = p.borrow_mut().p.take();

        node.borrow_mut().p = pp.clone();

        if ps == -1 {
            let pp = pp.unwrap();

            pp.borrow_mut().l = Some(node.clone());
        } else if ps == 1 {
            let pp = pp.unwrap();

            pp.borrow_mut().r = Some(node.clone());
        }

        let c;

        if s == -1 {
            c = node.borrow_mut().r.take();

            p.borrow_mut().l = c.clone();

            node.borrow_mut().r = Some(p.clone());
        } else {
            c = node.borrow_mut().l.take();

            p.borrow_mut().r = c.clone();

            node.borrow_mut().l = Some(p.clone());
        }

        if let Some(c) = c.as_ref() {
            c.borrow_mut().p = Some(p.clone());
        }

        p.borrow_mut().p = Some(node.clone());

        p.borrow_mut().update();

        node.borrow_mut().update();
    }

    pub fn splay(node: &N<T>) {
        loop {
            let s = node.borrow().state();

            if s == 0 {
                break;
            }

            let p = node.borrow().p.as_ref().unwrap().clone();

            let ps = p.borrow().state();

            if s * ps == 1 {
                Self::rotate(&p);
            } else if s * ps == -1 {
                Self::rotate(node);
            }

            Self::rotate(node);
        }
    }

    pub fn get(
        mut root: N<T>,
        mut i: usize,
    ) -> N<T> {
        loop {
            assert!(i < root.borrow().size);

            let lsize = root.borrow().lsize();

            if i < lsize {
                let l = root.borrow().l.as_ref().unwrap().clone();

                root = l;
            } else if i > lsize {
                let r = root.borrow().r.as_ref().unwrap().clone();

                root = r;

                i -= lsize + 1;
            } else {
                Self::splay(&root);

                return root;
            }
        }
    }

    pub fn merge(
        l: ON<T>,
        r: ON<T>,
    ) -> ON<T> {
        if r.is_none() {
            return l;
        }

        let mut r = r.unwrap();

        r = Self::get(r, 0);

        if let Some(l) = l.as_ref() {
            l.borrow_mut().p = Some(r.clone());
        }

        r.borrow_mut().l = l;

        r.borrow_mut().update();

        Some(r)
    }

    pub fn split(
        root: ON<T>,
        i: usize,
    ) -> (ON<T>, ON<T>) {
        let size = Self::size(root.as_ref());

        assert!(i <= size);

        if i == size {
            return (root, None);
        }

        let mut root = root.unwrap();

        root = Self::get(root, i);

        let l = root.borrow_mut().l.take();

        if let Some(l) = l.as_ref() {
            l.borrow_mut().p = None;
        }

        root.borrow_mut().update();

        (l, Some(root))
    }

    pub fn insert(
        root: ON<T>,
        i: usize,
        node: ON<T>,
    ) -> ON<T> {
        assert!(i <= Self::size(root.as_ref()));

        let (l, r) = Self::split(root, i);

        Self::merge(Self::merge(l, node), r)
    }

    pub fn pop(
        mut root: N<T>,
        i: usize,
    ) -> (N<T>, ON<T>) {
        root = Self::get(root, i);

        let l = root.borrow_mut().l.take();

        let r = root.borrow_mut().r.take();

        if let Some(l) = l.as_ref() {
            l.borrow_mut().p = None;
        }

        if let Some(r) = r.as_ref() {
            r.borrow_mut().p = None;
        }

        root.borrow_mut().update();

        let c = Self::merge(l, r);

        (root, c)
    }

    pub fn binary_search<F>(
        f: F,
        root: ORN<T>,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        if f(&root.borrow().v) {
            Self::binary_search(f, root.borrow().l.as_ref())
        } else {
            let offset = root.borrow().lsize() + 1;

            offset + Self::binary_search(f, root.borrow().r.as_ref())
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
