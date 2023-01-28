use std::collections::{
    BinaryHeap,
    HashMap,
};

pub struct LazyBinaryHeap<T> {
    que: BinaryHeap<T>,
    cnt: HashMap<T, isize>,
    size: usize,
}

impl<T: Ord + std::hash::Hash + Clone> LazyBinaryHeap<T> {
    pub fn new() -> Self {
        Self { que: BinaryHeap::new(), cnt: HashMap::new(), size: 0 }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn count(
        &self,
        x: &T,
    ) -> isize {
        *self.cnt.get(&x).or_else(|| Some(&0)).unwrap()
    }

    pub fn contains(
        &self,
        x: &T,
    ) -> bool {
        self.count(x) > 0
    }

    /// if count become negative,
    /// size are not changed as it as negative.
    /// if cnt = -5, heap size would't be changed until inserting at least 6.

    pub fn add(
        &mut self,
        x: T,
        delta: isize,
    ) {
        let c = self.cnt.entry(x.clone()).or_insert(0);

        let nc = *c + delta;

        if *c <= 0 && nc > 0 {
            self.que.push(x);
        }

        if delta >= 0 {
            self.size += nc.max(0).min(delta) as usize;
        } else {
            self.size -= (*c).max(0).min(-delta) as usize;
        }

        *c = nc;
    }

    pub fn insert(
        &mut self,
        x: T,
    ) {
        self.add(x, 1);
    }

    pub fn remove(
        &mut self,
        x: T,
    ) {
        self.add(x, -1);
    }

    fn lazy_discard_false_peek(&mut self) {
        while let Some(x) = self.que.peek() {
            if self.count(x) <= 0 {
                self.que.pop();

                continue;
            }

            break;
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.lazy_discard_false_peek();

        self.que.peek()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = LazyBinaryHeap::new();

        que.add(1, 5);

        assert_eq!(que.size(), 5);

        assert_eq!(que.peek(), Some(&1));

        que.add(2, 1);

        assert_eq!(que.size(), 6);

        assert_eq!(que.peek(), Some(&2));

        que.add(1, -6);

        assert_eq!(que.size(), 1);

        assert_eq!(que.count(&1), -1);

        que.add(1, 1);

        assert_eq!(que.size(), 1);

        que.add(2, -1);

        assert_eq!(que.size(), 0);

        assert_eq!(que.peek(), None);
    }
}
