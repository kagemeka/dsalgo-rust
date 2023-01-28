#[derive(Debug)]

pub struct Node<T> {
    // e: [Option<NonNull<Self>>; 3],
    e: [*mut Self; 3],
    size: usize,
    pub(crate) v: T,
}

use std::ptr::{
    eq,
    null_mut,
};

impl<T> Node<T> {
    pub fn new(v: T) -> Self { Self { e: [null_mut(); 3], size: 1, v } }

    pub(crate) fn size(root: *const Self) -> usize {
        unsafe { root.as_ref() }.map_or(0, |root| root.size)
    }

    fn update(&mut self) {
        self.size = 1;

        for i in 0..2 {
            self.size += Self::size(self.e[i]);
        }
    }

    fn state(&self) -> usize {
        unsafe { self.e[2].as_ref() }.map_or(2, |p| {
            if eq(p.e[0], self) {
                0
            } else {
                1
            }
        })
    }

    fn rotate(&mut self) {
        let s = self.state();

        assert!(s < 2);

        let p = unsafe { self.e[2].as_mut().unwrap() };

        let ps = p.state();

        let pp = p.e[2];

        self.e[2] = pp;

        if ps != 2 {
            unsafe { pp.as_mut().unwrap() }.e[ps] = self;
        }

        let c = self.e[s ^ 1];

        p.e[s] = c;

        self.e[s ^ 1] = p;

        if let Some(c) = unsafe { c.as_mut() } {
            c.e[2] = p;
        }

        p.e[2] = self;

        p.update();

        self.update();
    }

    fn splay(&mut self) {
        loop {
            let s = self.state();

            if s == 2 {
                break;
            }

            let p = unsafe { self.e[2].as_mut().unwrap() };

            let ps = p.state();

            if s ^ ps == 0 {
                p.rotate();
            } else if s ^ ps == 1 {
                self.rotate();
            }

            self.rotate();
        }
    }

    pub fn get(
        &mut self,
        mut i: usize,
    ) -> &mut Self {
        let mut node = self;

        loop {
            assert!(i < node.size);

            let lsize = Self::size(node.e[0]);

            if i < lsize {
                node = unsafe { node.e[0].as_mut().unwrap() };
            } else if i > lsize {
                node = unsafe { node.e[1].as_mut().unwrap() };

                i -= lsize + 1;
            } else {
                node.splay();

                return node;
            }
        }
    }

    pub fn merge(
        l: *mut Self,
        r: *mut Self,
    ) -> *mut Self {
        if r.is_null() {
            return l;
        }

        let r = unsafe { r.as_mut().unwrap() }.get(0);

        if let Some(l) = unsafe { l.as_mut() } {
            l.e[2] = r;
        }

        r.e[0] = l;

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

        let root = unsafe { root.as_mut().unwrap() }.get(i);

        let l = root.e[0];

        root.e[0] = null_mut();

        if let Some(l) = unsafe { l.as_mut() } {
            l.e[2] = null_mut();
        }

        root.update();

        (l, root)
    }

    pub fn insert(
        root: *mut Self,
        i: usize,
        node: *mut Self,
    ) -> *mut Self {
        let (l, r) = Self::split(root, i);

        Self::merge(Self::merge(l, node), r)
    }

    pub fn pop(
        &mut self,
        i: usize,
    ) -> (&mut Self, *mut Self) {
        let root = self.get(i);

        let l = root.e[0];

        let r = root.e[1];

        root.e[0] = null_mut();

        root.e[1] = null_mut();

        if let Some(l) = unsafe { l.as_mut() } {
            l.e[2] = null_mut();
        }

        if let Some(r) = unsafe { r.as_mut() } {
            r.e[2] = null_mut();
        }

        root.update();

        (root, Self::merge(l, r))
    }

    pub fn binary_search<F>(
        f: F,
        mut root: *mut Self,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        let mut i = 0;

        while let Some(node) = unsafe { root.as_ref() } {
            if f(&node.v) {
                root = node.e[0];
            } else {
                root = node.e[1];

                i += Self::size(node.e[0]) + 1;
            }
        }

        i
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
