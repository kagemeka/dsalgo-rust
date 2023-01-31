#[derive(Debug)]

pub struct Node<T> {
    c: [*mut Self; 2],
    size: usize,
    pub v: T,
}

use std::ptr::null_mut;

impl<T> Node<T> {
    pub fn new(v: T) -> Self { Self { c: [null_mut(); 2], size: 1, v } }

    pub(crate) fn size(root: *const Self) -> usize {
        unsafe { root.as_ref() }.map_or(0, |root| root.size)
    }

    fn c_size(
        &self,
        i: usize,
    ) -> usize {
        Self::size(self.c[i])
    }

    fn update(&mut self) {
        self.size = 1;

        for &c in self.c.iter() {
            self.size += Self::size(c);
        }
    }

    fn rotate_up(
        &mut self,
        i: usize,
    ) -> &mut Self {
        debug_assert!(i < 2);

        let mut c = unsafe { self.c[i].as_mut().unwrap() };

        self.c[i] = c.c[i ^ 1];

        self.update();

        c.c[i ^ 1] = self;

        c.update();

        c
    }

    fn state(
        &self,
        k: usize,
    ) -> usize {
        let lsize = self.c_size(0);

        if k < lsize {
            0
        } else if k > lsize {
            1
        } else {
            2 // itself
        }
    }

    pub fn splay(
        &mut self,
        mut k: usize,
    ) -> &mut Self {
        let mut node = self;

        assert!(k < node.size);

        let s = node.state(k);

        if s == 2 {
            return node;
        }

        let mut c = unsafe { node.c[s].as_mut().unwrap() };

        k -= (node.c_size(0) + 1) * s;

        let cs = c.state(k);

        if cs == 2 {
            node.c[s] = c;
        } else {
            k -= (c.c_size(0) + 1) * cs;

            c.c[cs] = unsafe { c.c[cs].as_mut().unwrap() }.splay(k);

            if s ^ cs == 0 {
                node.c[s] = c;

                node = node.rotate_up(s);
            } else {
                node.c[s] = c.rotate_up(cs);
            }
        }

        node.rotate_up(s)
    }

    pub fn merge(
        l: *mut Self,
        r: *mut Self,
    ) -> *mut Self {
        if r.is_null() {
            return l;
        }

        let mut r = unsafe { r.as_mut().unwrap() }.splay(0);

        r.c[0] = l;

        r.update();

        r
    }

    pub fn split(
        root: *mut Self,
        i: usize,
    ) -> (*mut Self, *mut Self) {
        let size = Self::size(root);

        assert!(i <= size);

        if i == size {
            return (root, null_mut());
        }

        let mut root = unsafe { root.as_mut().unwrap() }.splay(i);

        let l = root.c[0];

        root.c[0] = null_mut();

        root.update();

        (l, root)
    }

    pub fn insert(
        root: *mut Self,
        i: usize,
        node: *mut Self,
    ) -> *mut Self {
        assert!(i <= Self::size(root));

        let (l, r) = Self::split(root, i);

        Self::merge(Self::merge(l, node), r)
    }

    pub fn pop(
        &mut self,
        i: usize,
    ) -> (&mut Self, *mut Self) {
        let mut node = self;

        node = Self::splay(node, i);

        let c = Self::merge(node.c[0], node.c[1]);

        node.c[0] = null_mut();

        node.c[1] = null_mut();

        (node, c)
    }

    pub fn binary_search<F>(
        f: F,
        root: *const Self,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if root.is_null() {
            return 0;
        }

        let root = unsafe { root.as_ref().unwrap() };

        if f(&root.v) {
            Self::binary_search(f, root.c[0])
        } else {
            root.c_size(0) + 1 + Self::binary_search(f, root.c[1])
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
