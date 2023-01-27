#[derive(Debug)]

pub struct Node<T> {
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub height: usize,
    pub size: usize,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Self> {
        Box::new(Self { left: None, right: None, height: 1, size: 1, value })
    }

    pub(crate) fn height(node: Option<&Box<Self>>) -> usize {
        if let Some(node) = node {
            node.height
        } else {
            0
        }
    }

    pub(crate) fn size(node: Option<&Box<Self>>) -> usize {
        if let Some(node) = node {
            node.size
        } else {
            0
        }
    }

    pub(crate) fn balance(node: Option<&Box<Self>>) -> isize {
        if let Some(node) = node {
            Self::height(node.right.as_ref()) as isize
                - Self::height(node.left.as_ref()) as isize
        } else {
            0
        }
    }

    pub(crate) fn update(root: &mut Box<Self>) {
        let hl = Self::height(root.left.as_ref());

        let hr = Self::height(root.right.as_ref());

        root.height = hl.max(hr) + 1;

        let sl = Self::size(root.left.as_ref());

        let sr = Self::size(root.right.as_ref());

        root.size = sl + sr + 1;
    }

    pub(crate) fn rotate_left(mut root: Box<Self>) -> Box<Self> {
        let mut new_root = root.right.take().unwrap();

        root.right = new_root.left.take();

        Self::update(&mut root);

        new_root.left = Some(root);

        Self::update(&mut new_root);

        new_root
    }

    pub(crate) fn rotate_right(mut root: Box<Self>) -> Box<Self> {
        let mut new_root = root.left.take().unwrap();

        root.left = new_root.right.take();

        Self::update(&mut root);

        new_root.right = Some(root);

        Self::update(&mut new_root);

        new_root
    }

    pub(crate) fn rebalance(mut root: Box<Self>) -> Box<Self> {
        Self::update(&mut root);

        let b = Self::balance(Some(&root));

        if b < -1 {
            if Self::balance(root.left.as_ref()) > 0 {
                root.left = Some(Self::rotate_left(root.left.take().unwrap()));
            }

            return Self::rotate_right(root);
        } else if b > 1 {
            if Self::balance(root.right.as_ref()) < 0 {
                root.right =
                    Some(Self::rotate_right(root.right.take().unwrap()));
            }

            return Self::rotate_left(root);
        }

        root
    }

    pub fn insert(
        root: Option<Box<Self>>,
        i: usize,
        node: Box<Self>,
    ) -> Box<Self> {
        assert!(i <= Self::size(root.as_ref()));

        if root.is_none() {
            return node;
        }

        let mut root = root.unwrap();

        let lsize = Self::size(root.left.as_ref());

        if i <= lsize {
            root.left = Some(Self::insert(root.left.take(), i, node));
        } else {
            root.right =
                Some(Self::insert(root.right.take(), i - lsize - 1, node));
        }

        Self::rebalance(root)
    }

    pub(crate) fn pop_last(
        mut root: Box<Self>
    ) -> (Box<Self>, Option<Box<Self>>) {
        if root.right.is_none() {
            let new_root = root.left.take();

            return (root, new_root);
        }

        let (max_node, new_right) = Self::pop_last(root.right.take().unwrap());

        root.right = new_right;

        (max_node, Some(Self::rebalance(root)))
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
            if root.left.is_none() {
                return root.right;
            }

            let (max_node, new_left) =
                Self::pop_last(root.left.take().unwrap());

            root.left = new_left;

            root.value = max_node.value;
        }

        Some(Self::rebalance(root))
    }

    pub fn kth_node(
        root: &Box<Self>,
        k: usize,
    ) -> &Box<Self> {
        assert!(k < root.size);

        let lsize = Self::size(root.left.as_ref());

        if k < lsize {
            Self::kth_node(root.left.as_ref().unwrap(), k)
        } else if k > lsize {
            Self::kth_node(root.right.as_ref().unwrap(), k - lsize - 1)
        } else {
            root
        }
    }

    pub fn binary_search<F>(
        f: F,
        root: Option<&Box<Self>>,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        if f(&root.value) {
            Self::binary_search(f, root.left.as_ref())
        } else {
            let offset = Self::size(root.left.as_ref()) + 1;

            offset + Self::binary_search(f, root.right.as_ref())
        }
    }

    pub fn iter<'a>(&'a self) -> std::vec::IntoIter<&'a T> {
        let mut inorder = vec![];

        fn dfs<'b, T>(
            res: &mut Vec<&'b T>,
            node: &'b Node<T>,
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

impl<T> IntoIterator for Node<T> {
    type IntoIter = std::vec::IntoIter<Self::Item>;

    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        let mut inorder = vec![];

        fn dfs<T>(
            res: &mut Vec<T>,
            mut node: Node<T>,
        ) {
            if let Some(left) = node.left.take() {
                dfs(res, *left);
            }

            res.push(node.value);

            if let Some(right) = node.right.take() {
                dfs(res, *right);
            }
        }

        dfs(&mut inorder, self);

        inorder.into_iter()
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

        root = Some(Nd::insert(root, 1, Nd::new(2)));

        println!("{:?}", root);

        root = Some(Nd::insert(root, 2, Nd::new(3)));

        println!("{:?}", root);

        root = Nd::remove(root.unwrap(), 1);

        println!("{:?}", root);

        root = Nd::remove(root.unwrap(), 0);

        println!("{:?}", root);

        assert_eq!(Nd::kth_node(root.as_ref().unwrap(), 0).value, 3);

        root = Some(Nd::insert(root, 1, Nd::new(1)));

        println!("{:?}", root);

        assert_eq!(Nd::binary_search(|v| v <= &1, root.as_ref()), 1);

        assert_eq!(Nd::binary_search(|v| v < &1, root.as_ref()), 2);

        assert!(!std::ptr::eq(&Nd::new(1), &Nd::new(1)));

        for v in root.as_ref().unwrap().iter() {
            println!("{:?}", v);
        }

        for v in root.as_ref().unwrap().iter() {
            println!("{:?}", v);
        }

        for v in root.unwrap().into_iter() {
            println!("{:?}", v);
        }
    }
}
