/// store cs with array
/// have subtree size

pub struct Node<T> {
    c: [ON<T>; 2],
    size: usize,
    pub v: T,
}

type N<T> = Box<Node<T>>;

type ON<T> = Option<N<T>>;

type ORN<'a, T> = Option<&'a N<T>>;

impl<T> Node<T> {
    pub fn new(v: T) -> N<T> { Box::new(Self { c: [None, None], size: 1, v }) }

    pub(crate) fn size(root: ORN<T>) -> usize {
        if let Some(root) = root {
            root.size
        } else {
            0
        }
    }

    fn c_size(
        &self,
        i: usize,
    ) -> usize {
        Self::size(self.c[i].as_ref())
    }

    fn update(&mut self) {
        self.size = 1;

        for c in self.c.iter() {
            self.size += Self::size(c.as_ref());
        }
    }

    // i = 0 -> up left, 1 -> up right
    fn rotate_up(
        mut root: N<T>,
        i: usize,
    ) -> N<T> {
        debug_assert!(i == 0 || i == 1);

        let mut c = root.c[i].take().unwrap();

        root.c[i] = c.c[i ^ 1].take();

        root.update();

        c.c[i ^ 1] = Some(root);

        c.update();

        c
    }

    // which direction kth node exist?
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
        mut root: N<T>,
        mut k: usize,
    ) -> N<T> {
        assert!(k < root.size);

        let s = root.state(k);

        if s == 2 {
            return root;
        }

        let mut c = root.c[s].take().unwrap();

        k -= (root.c_size(0) + 1) * s;

        let cs = c.state(k);

        if cs == 2 {
            root.c[s] = Some(c);
        } else {
            k -= (c.c_size(0) + 1) * cs;

            if s ^ cs == 0 {
                // zig-zig or zag-zag
                c.c[cs] = Some(Self::splay(c.c[cs].take().unwrap(), k));

                root.c[s] = Some(c);

                root = Self::rotate_up(root, s);
            } else {
                c.c[cs] = Some(Self::splay(c.c[cs].take().unwrap(), k));

                root.c[s] = Some(Self::rotate_up(c, cs));
            }
        }

        Self::rotate_up(root, s)
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

        r.c[0] = l;

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

        let l = root.c[0].take();

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

        let c = Self::merge(root.c[0].take(), root.c[1].take());

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
            Self::binary_search(f, root.c[0].as_ref())
        } else {
            let offset = root.c_size(0) + 1;

            offset + Self::binary_search(f, root.c[1].as_ref())
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
