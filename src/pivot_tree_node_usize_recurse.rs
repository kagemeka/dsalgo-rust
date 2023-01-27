pub struct Node {
    pivot: usize,
    value: usize,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl Node {
    /// capacity: 2^max_height - 1
    /// 1 <= value < 2^max_height internally,

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
        }))
    }

    fn with_pivot(
        pivot: usize,
        value: usize,
    ) -> Option<Box<Self>> {
        Some(Box::new(Self { pivot, value, left: None, right: None }))
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

        if v < self.value {}

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
    }

    pub fn remove(
        root: Option<Box<Self>>,
        v: usize,
    ) -> Option<Box<Self>> {
        if root.is_none() {
            return None;
        }

        let mut root = root.unwrap();

        if v < root.value {
            root.left = Self::remove(root.left.take(), v);
        } else if v > root.value {
            root.right = Self::remove(root.right.take(), v);
        } else {
            if let Some(v) = Self::min(root.right.as_ref()) {
                root.value = v;

                root.right = Self::remove(root.right.take(), v);
            } else if let Some(v) = Self::max(root.left.as_ref()) {
                root.value = v;

                root.left = Self::remove(root.left.take(), v);
            } else {
                return None;
            }
        }

        Some(root)
    }

    pub fn min(root: Option<&Box<Self>>) -> Option<usize> {
        if let Some(root) = root {
            Some(if let Some(mn) = Self::min(root.left.as_ref()) {
                mn
            } else {
                root.value
            })
        } else {
            None
        }
    }

    pub fn max(root: Option<&Box<Self>>) -> Option<usize> {
        if let Some(root) = root {
            Some(if let Some(mx) = Self::max(root.right.as_ref()) {
                mx
            } else {
                root.value
            })
        } else {
            None
        }
    }

    pub fn min_ok<F>(
        is_ok: F,
        root: Option<&Box<Self>>,
    ) -> Option<usize>
    where
        F: Fn(usize) -> bool,
    {
        if root.is_none() {
            return None;
        }

        let root = root.unwrap();

        if is_ok(root.value) {
            Some(if let Some(v) = Self::min_ok(is_ok, root.left.as_ref()) {
                v
            } else {
                root.value
            })
        } else {
            Self::min_ok(is_ok, root.right.as_ref())
        }
    }

    pub fn max_ok<F>(
        is_ok: F,
        root: Option<&Box<Self>>,
    ) -> Option<usize>
    where
        F: Fn(usize) -> bool,
    {
        if root.is_none() {
            return None;
        }

        let root = root.unwrap();

        if is_ok(root.value) {
            Some(if let Some(v) = Self::max_ok(is_ok, root.right.as_ref()) {
                v
            } else {
                root.value
            })
        } else {
            Self::max_ok(is_ok, root.left.as_ref())
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

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
