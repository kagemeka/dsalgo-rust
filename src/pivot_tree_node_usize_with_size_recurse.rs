#[derive(Debug)]

pub struct Node {
    pivot: usize,
    pub(crate) value: usize,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    size: usize,
}

impl Node {
    /// capacity: 1 <= value < 2^max_height
    /// 0 <= value < 2^max_height - 1 internally,

    pub fn new(
        max_height: usize,
        value: usize,
    ) -> Option<Box<Self>> {
        assert!(max_height > 0);

        assert!(1 <= value && value < 1 << max_height);

        Some(Box::new(Self {
            pivot: 1 << (max_height - 1),
            value,
            left: None,
            right: None,
            size: 1,
        }))
    }

    fn with_pivot(
        pivot: usize,
        value: usize,
    ) -> Option<Box<Self>> {
        Some(Box::new(Self { pivot, value, left: None, right: None, size: 1 }))
    }

    pub(crate) fn size(node: Option<&Box<Self>>) -> usize {
        if let Some(node) = node {
            node.size
        } else {
            0
        }
    }

    pub fn update(&mut self) {
        self.size = Self::size(self.left.as_ref())
            + Self::size(self.right.as_ref())
            + 1;
    }

    pub fn insert(
        &mut self,
        mut v: usize,
    ) {
        use std::mem::swap;

        assert!(v != self.value);

        let p = self.pivot;

        let d = 1 << p.trailing_zeros();

        assert!(d > 1 && p - d < v && v < p + d);

        if self.value.min(v) < self.pivot {
            if self.value < v {
                swap(&mut self.value, &mut v);
            }

            if let Some(l) = self.left.as_mut() {
                l.insert(v);
            } else {
                self.left = Self::with_pivot(p - (d >> 1), v);
            }
        } else {
            if self.value > v {
                swap(&mut self.value, &mut v);
            }

            if let Some(r) = self.right.as_mut() {
                r.insert(v);
            } else {
                self.right = Self::with_pivot(p + (d >> 1), v);
            }
        }

        self.update();
    }

    pub fn remove(
        mut root: Box<Self>,
        i: usize,
    ) -> Option<Box<Self>> {
        assert!(i < root.size);

        let lsize = Self::size(root.left.as_ref());

        if i < lsize {
            root.left = Self::remove(root.left.take().unwrap(), i);
        } else if i > lsize {
            root.right =
                Self::remove(root.right.take().unwrap(), i - lsize - 1);
        } else {
            if let Some(right) = root.right.take() {
                root.value = right.kth_node(0).value;

                root.right = Self::remove(right, 0);
            } else if let Some(left) = root.left.take() {
                root.value = left.kth_node(lsize - 1).value;

                root.left = Self::remove(left, lsize - 1);
            } else {
                return None;
            }
        }

        root.update();

        Some(root)
    }

    pub fn kth_node(
        &self,
        k: usize,
    ) -> &Self {
        assert!(k < self.size);

        let lsize = Self::size(self.left.as_ref());

        if k < lsize {
            self.left.as_ref().unwrap().kth_node(k)
        } else if k == lsize {
            self
        } else {
            self.right.as_ref().unwrap().kth_node(k - lsize - 1)
        }
    }

    pub fn binary_search<F>(
        f: F,
        root: Option<&Box<Self>>,
    ) -> usize
    where
        F: Fn(usize) -> bool,
    {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        if f(root.value) {
            Self::binary_search(f, root.left.as_ref())
        } else {
            let offset = Self::size(root.left.as_ref()) + 1;

            offset + Self::binary_search(f, root.right.as_ref())
        }
    }

    pub fn iter<'a>(&'a self) -> std::vec::IntoIter<&'a usize> {
        let mut inorder = vec![];

        fn dfs<'b>(
            res: &mut Vec<&'b usize>,
            node: &'b Node,
        ) {
            if let Some(left) = node.left.as_ref() {
                dfs(res, left);
            }

            res.push(&node.value);

            if let Some(right) = node.right.as_ref() {
                dfs(res, right);
            }
        }

        dfs(&mut inorder, self);

        inorder.into_iter()
    }
}
