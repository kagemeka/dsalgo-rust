use std::{
    cmp::Reverse,
    collections::{
        BinaryHeap,
        HashMap,
    },
};

pub struct MinMaxQueue<T> {
    min_que: BinaryHeap<Reverse<T>>,
    max_que: BinaryHeap<T>,
    cnt: HashMap<T, usize>,
    size: usize,
}

impl<T: Ord + std::hash::Hash + Clone> MinMaxQueue<T> {
    pub fn new() -> Self {
        Self {
            min_que: BinaryHeap::new(),
            max_que: BinaryHeap::new(),
            cnt: HashMap::new(),
            size: 0,
        }
    }

    pub fn insert(
        &mut self,
        x: T,
        count: usize,
    ) {
        assert!(count > 0);

        let c = self.cnt.entry(x.clone()).or_insert(0);

        if *c == 0 {
            self.min_que.push(Reverse(x.clone()));

            self.max_que.push(x.clone());
        }

        *c += count;

        self.size += count;
    }

    pub fn size(&self) -> usize { self.size }

    pub fn count(
        &self,
        x: &T,
    ) -> usize {
        *self.cnt.get(x).or_else(|| Some(&0)).unwrap()
    }

    pub fn contains(
        &self,
        x: &T,
    ) -> bool {
        self.count(x) > 0
    }

    pub fn remove(
        &mut self,
        x: &T,
        count: usize,
    ) {
        assert!(self.count(x) >= count && count > 0);

        *self.cnt.get_mut(&x).unwrap() -= count;

        self.size -= count;
    }

    fn lazy_discard_false_min(&mut self) {
        while let Some(Reverse(x)) = self.min_que.peek() {
            if self.count(x) == 0 {
                self.min_que.pop();

                continue;
            }

            break;
        }
    }

    fn lazy_discard_false_max(&mut self) {
        while let Some(x) = self.max_que.peek() {
            if self.count(x) == 0 {
                self.max_que.pop();

                continue;
            }

            break;
        }
    }

    fn min(&mut self) -> Option<&T> {
        self.lazy_discard_false_min();

        if let Some(Reverse(x)) = self.min_que.peek() {
            Some(x)
        } else {
            None
        }
    }

    fn max(&mut self) -> Option<&T> {
        self.lazy_discard_false_max();

        self.max_que.peek()
    }

    pub fn pop_min(&mut self) -> Option<T> {
        let v = self.min();

        if v.is_none() {
            return None;
        }

        let v = v.unwrap().clone();

        assert!(self.count(&v) > 0);

        *self.cnt.get_mut(&v).unwrap() -= 1;

        self.size -= 1;

        Some(v)
    }

    pub fn pop_max(&mut self) -> Option<T> {
        let v = self.max();

        if v.is_none() {
            return None;
        }

        let v = v.unwrap().clone();

        assert!(self.count(&v) > 0);

        *self.cnt.get_mut(&v).unwrap() -= 1;

        self.size -= 1;

        Some(v)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = MinMaxQueue::new();

        que.insert(1, 5);

        assert_eq!(que.min(), Some(&1));

        assert_eq!(que.size(), 5);

        que.insert(-1, 1);

        assert_eq!(que.min(), Some(&-1));

        assert_eq!(que.max(), Some(&1));

        assert_eq!(que.size(), 6);

        assert_eq!(que.pop_max(), Some(1));

        que.remove(&1, 3);

        assert_eq!(que.size(), 2);

        assert_eq!(que.pop_min(), Some(-1));
    }
}
