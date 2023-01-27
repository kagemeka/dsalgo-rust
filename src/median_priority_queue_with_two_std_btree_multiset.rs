use crate::btree_multiset::Multiset;

pub struct MedianQueue<T> {
    lo: Multiset<T>,
    hi: Multiset<T>,
}

impl<T: Ord + Clone> MedianQueue<T> {
    pub fn new() -> Self { Self { lo: Multiset::new(), hi: Multiset::new() } }

    fn balance(&self) -> isize {
        self.lo.size() as isize - self.hi.size() as isize
    }

    fn rebalance(&mut self) {
        let b = self.balance();

        if b == 2 {
            let x = self.lo.max().unwrap().clone();

            self.lo.remove(&x, 1);

            self.hi.insert(x, 1);
        } else if b == -1 {
            let x = self.hi.min().unwrap().clone();

            self.hi.remove(&x, 1);

            self.lo.insert(x, 1);
        }
    }

    pub fn size(&self) -> usize { self.lo.size() + self.hi.size() }

    pub fn count(
        &self,
        x: &T,
    ) -> usize {
        self.lo.count(x) + self.hi.count(x)
    }

    pub fn insert(
        &mut self,
        x: T,
    ) {
        if self.balance() == 1 {
            self.lo.insert(x, 1);
        } else {
            self.hi.insert(x, 1);
        }

        self.rebalance();
    }

    pub fn low(&self) -> Option<&T> { self.lo.max() }

    pub fn high(&self) -> Option<&T> {
        if self.balance() == 1 {
            self.low()
        } else {
            self.hi.min()
        }
    }

    pub fn remove(
        &mut self,
        x: &T,
    ) {
        assert!(self.count(x) > 0);

        if self.lo.count(x) > 0 {
            self.lo.remove(x, 1);
        } else {
            self.hi.remove(x, 1);
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
