/// have subtree size

pub struct Node<T> {
    l: ON<T>,
    r: ON<T>,
    size: usize,
    pub v: T,
}

type N<T> = Box<Node<T>>;

type ON<T> = Option<N<T>>;

type ORN<'a, T> = Option<&'a N<T>>;

impl<T> Node<T> {
    pub fn new(v: T) -> N<T> { Box::new(Self { l: None, r: None, size: 1, v }) }

    pub(crate) fn size(root: ORN<T>) -> usize {
        if let Some(root) = root {
            root.size
        } else {
            0
        }
    }

    fn lsize(&self) -> usize { Self::size(self.l.as_ref()) }

    fn update(&mut self) {
        let lsize = Self::size(self.l.as_ref());

        let rsize = Self::size(self.r.as_ref());

        self.size = lsize + rsize + 1;
    }

    // counter-clock-wise
    fn rot_l(mut root: N<T>) -> N<T> {
        let mut c = root.r.take().unwrap();

        root.r = c.l.take();

        root.update();

        c.l = Some(root);

        c.update();

        c
    }

    // clock-wise
    fn rot_r(mut root: N<T>) -> N<T> {
        let mut c = root.l.take().unwrap();

        root.l = c.r.take();

        root.update();

        c.r = Some(root);

        c.update();

        c
    }

    fn state(
        &self,
        k: usize,
    ) -> isize {
        let lsize = self.lsize();

        if k < lsize {
            -1
        } else if k == lsize {
            0
        } else {
            1
        }
    }

    // bring k-th node to the root
    pub fn splay(
        mut root: N<T>,
        mut k: usize,
    ) -> N<T> {
        assert!(k < root.size);

        let s = root.state(k);

        if s == 0 {
            return root;
        }

        if s == -1 {
            let mut c = root.l.take().unwrap();

            let cs = c.state(k);

            if cs == 0 {
                root.l = Some(c);
            } else if s * cs == 1 {
                c.l = Some(Self::splay(c.l.take().unwrap(), k));

                root.l = Some(c);

                root = Self::rot_r(root);
            } else {
                k -= c.lsize() + 1;

                c.r = Some(Self::splay(c.r.take().unwrap(), k));

                c = Self::rot_l(c);

                root.l = Some(c);
            }

            Self::rot_r(root)
        } else {
            k -= root.lsize() + 1;

            let mut c = root.r.take().unwrap();

            let cs = c.state(k);

            if cs == 0 {
                root.r = Some(c);
            } else if s * cs == 1 {
                k -= c.lsize() + 1;

                c.r = Some(Self::splay(c.r.take().unwrap(), k));

                root.r = Some(c);

                root = Self::rot_l(root);
            } else {
                c.l = Some(Self::splay(c.l.take().unwrap(), k));

                c = Self::rot_r(c);

                root.r = Some(c);
            }

            Self::rot_l(root)
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

        r = Self::splay(r, 0);

        r.l = l;

        r.update();

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

        root = Self::splay(root, i);

        let l = root.l.take();

        root.update();

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
        root = Self::splay(root, i);

        let c = Self::merge(root.l.take(), root.r.take());

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

        if f(&root.v) {
            Self::binary_search(f, root.l.as_ref())
        } else {
            let offset = root.lsize() + 1;

            offset + Self::binary_search(f, root.r.as_ref())
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
