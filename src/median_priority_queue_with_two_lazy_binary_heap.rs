use std::cmp::Reverse;

use crate::lazy_binary_heap_with_count_hash::LazyBinaryHeap;

pub struct MedianQueue<T> {
    lo: LazyBinaryHeap<T>,
    hi: LazyBinaryHeap<Reverse<T>>,
}

impl<T: Ord + Clone + std::hash::Hash> MedianQueue<T> {
    pub fn new() -> Self {
        Self { lo: LazyBinaryHeap::new(), hi: LazyBinaryHeap::new() }
    }

    fn balance(&self) -> isize {
        self.lo.size() as isize - self.hi.size() as isize
    }

    fn rebalance(&mut self) {
        let b = self.balance();

        if b == 2 {
            let x = self.lo.peek().unwrap().clone();

            self.lo.remove(x.clone());

            self.hi.insert(Reverse(x));
        } else if b == -1 {
            let x = self.hi.peek().unwrap().clone();

            self.hi.remove(x.clone());

            self.lo.insert(x.0);
        }
    }

    pub fn size(&self) -> usize { self.lo.size() + self.hi.size() }

    pub fn count(
        &self,
        x: &T,
    ) -> usize {
        self.lo.count(x) as usize + self.hi.count(&Reverse(x.clone())) as usize
    }

    pub fn insert(
        &mut self,
        x: T,
    ) {
        if self.balance() == 1 {
            self.lo.insert(x);
        } else {
            self.hi.insert(Reverse(x));
        }

        self.rebalance();
    }

    pub fn low(&mut self) -> Option<&T> { self.lo.peek() }

    pub fn high(&mut self) -> Option<&T> {
        if self.balance() == 1 {
            self.low()
        } else {
            if let Some(Reverse(x)) = self.hi.peek() {
                Some(x)
            } else {
                None
            }
        }
    }

    pub fn remove(
        &mut self,
        x: &T,
    ) {
        assert!(self.count(x) > 0);

        if self.lo.count(x) > 0 {
            self.lo.remove(x.clone());
        } else {
            self.hi.remove(Reverse(x.clone()));
        }

        self.rebalance();
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = MedianQueue::new();

        for i in 0..10 {
            que.insert(i);
        }

        assert_eq!(que.size(), 10);

        assert_eq!(que.low(), Some(&4));

        assert_eq!(que.high(), Some(&5));

        que.remove(&4);

        assert_eq!(que.low(), Some(&5));

        assert_eq!(que.high(), Some(&5));

        que.remove(&5);

        assert_eq!(que.low(), Some(&3));

        assert_eq!(que.high(), Some(&6));

        que.insert(4);

        assert_eq!(que.low(), Some(&4));

        assert_eq!(que.high(), Some(&4));
    }
}
