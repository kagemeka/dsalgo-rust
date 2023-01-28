//! functionality
//! push arbitrary,
//! pop median
//! find median,

use std::{
    cmp::Reverse,
    collections::BinaryHeap,
};

pub struct MedianQueue<T> {
    low_que: BinaryHeap<T>,
    high_que: BinaryHeap<Reverse<T>>,
}

impl<T: Clone + Ord> MedianQueue<T> {
    pub fn new() -> Self {
        Self { low_que: BinaryHeap::new(), high_que: BinaryHeap::new() }
    }

    pub fn size(&self) -> usize { self.low_que.len() + self.high_que.len() }

    fn low_to_high(&mut self) {
        self.high_que.push(Reverse(self.low_que.pop().unwrap()));
    }

    fn high_to_low(&mut self) {
        self.low_que.push(self.high_que.pop().unwrap().0);
    }

    fn balance(&self) -> isize {
        self.low_que.len() as isize - self.high_que.len() as isize
    }

    fn rebalance(&mut self) {
        match self.balance() {
            2 => self.low_to_high(),
            -1 => self.high_to_low(),
            0 | 1 => (),
            _ => panic!("cannot be"),
        }

        debug_assert!(self.balance() == 0 || self.balance() == 1);
    }

    pub fn push(
        &mut self,
        x: T,
    ) {
        if self.balance() == 1 {
            self.low_que.push(x);
        } else {
            self.high_que.push(Reverse(x));
        }

        self.rebalance();
    }

    pub fn low(&self) -> Option<&T> { self.low_que.peek() }

    pub fn high(&self) -> Option<&T> {
        if self.balance() == 1 {
            self.low_que.peek()
        } else if let Some(Reverse(x)) = self.high_que.peek() {
            Some(x)
        } else {
            None
        }
    }

    pub fn pop_low(&mut self) -> Option<T> {
        let v = self.low_que.pop();

        self.rebalance();

        v
    }

    pub fn pop_high(&mut self) -> Option<T> {
        let v = if self.balance() == 1 {
            self.low_que.pop()
        } else if let Some(Reverse(x)) = self.high_que.pop() {
            Some(x)
        } else {
            None
        };

        self.rebalance();

        v
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = MedianQueue::new();

        for i in 0..10 {
            que.push(i);
        }

        assert_eq!(que.size(), 10);

        assert_eq!(que.low(), Some(&4));

        assert_eq!(que.high(), Some(&5));

        assert_eq!(que.pop_low(), Some(4));

        assert_eq!(que.low(), Some(&5));

        assert_eq!(que.high(), Some(&5));

        assert_eq!(que.pop_high(), Some(5));

        assert_eq!(que.low(), Some(&3));

        assert_eq!(que.high(), Some(&6));

        que.push(4);

        assert_eq!(que.low(), Some(&4));

        assert_eq!(que.high(), Some(&4));
    }
}
