use crate::{
    binary_search_2_ok_usize::binary_search,
    integer_square_root_with_binary_search_usize::isqrt,
};

/// ref: https://github.com/tatyam-prime/SortedSet/blob/main/SortedMultiset.py
#[derive(Debug)]

pub struct SqrtBucket<T> {
    pub(crate) buckets: Vec<Vec<T>>,
    size: usize,
}

impl<T> SqrtBucket<T> {
    pub fn new() -> Self { Self { buckets: vec![vec![]], size: 0 } }

    pub fn size(&self) -> usize { self.size }

    fn max_interval(&self) -> usize {
        // isqrt(self.size) << 4;
        self.buckets.len() * 170
    }

    fn rebalance(&mut self) {
        use std::mem::swap;

        if self.size == 0 {
            assert!(self.buckets.len() == 1 && self.buckets[0].len() == 0);

            return;
        }

        // let s = isqrt(self.size) << 3;
        let s = isqrt(self.size) * 7;

        let m = (self.size + s - 1) / s;

        let mut b: Vec<_> = (0..m).map(|_| Vec::with_capacity(s)).collect();

        swap(&mut b, &mut self.buckets);

        for (i, x) in b.into_iter().flatten().enumerate() {
            self.buckets[i / s].push(x);
        }
    }

    /// false, false, ..., true, true

    pub(crate) fn search_bucket<F>(
        &self,
        is_ok: F,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        debug_assert!(self.buckets.len() > 0);

        binary_search(
            |j| is_ok(self.buckets[j].last().unwrap()),
            0,
            self.buckets.len() - 1,
        )
    }

    pub(crate) fn search_on_bucket<F>(
        &self,
        j: usize,
        is_ok: F,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        let b = &self.buckets[j];

        binary_search(|i| is_ok(&b[i]), 0, b.len())
    }

    pub(crate) fn search_node<F>(
        &self,
        is_ok: F,
    ) -> (usize, usize)
    where
        F: Fn(&T) -> bool,
    {
        let j = self.search_bucket(&is_ok);

        let k = self.search_on_bucket(j, &is_ok);

        (j, k)
    }

    pub(crate) fn index_of(
        &self,
        j: usize,
        k: usize,
    ) -> usize {
        let mut i = 0;

        for b in self.buckets[..j].iter() {
            i += b.len();
        }

        i + k
    }

    pub(crate) fn node_of(
        &self,
        mut i: usize,
    ) -> (usize, usize) {
        assert!(i <= self.size);

        for (j, b) in self.buckets[..self.buckets.len() - 1].iter().enumerate()
        {
            if i < b.len() {
                return (j, i);
            }

            i -= b.len();
        }

        (self.buckets.len() - 1, i)
    }

    pub fn binary_search<F>(
        &self,
        is_ok: F,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        let (j, k) = self.search_node(is_ok);

        self.index_of(j, k)
    }

    pub(crate) fn insert_at(
        &mut self,
        j: usize,
        k: usize,
        x: T,
    ) {
        self.buckets[j].insert(k, x);

        self.size += 1;

        if self.buckets[j].len() > self.max_interval() {
            self.rebalance();
        }
    }

    pub fn insert(
        &mut self,
        i: usize,
        x: T,
    ) {
        let (j, k) = self.node_of(i);

        self.insert_at(j, k, x);
    }

    pub(crate) fn remove_at(
        &mut self,
        j: usize,
        k: usize,
    ) {
        self.buckets[j].remove(k);

        self.size -= 1;

        if self.buckets[j].len() == 0 {
            self.rebalance();
        }
    }

    pub fn remove(
        &mut self,
        i: usize,
    ) {
        assert!(i < self.size);

        let (j, k) = self.node_of(i);

        self.remove_at(j, k);
    }
}

use std::ops::*;

impl<T> Index<usize> for SqrtBucket<T> {
    type Output = T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        let (j, k) = self.node_of(i);

        &self.buckets[j][k]
    }
}

impl<T> IndexMut<usize> for SqrtBucket<T> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        let (j, k) = self.node_of(i);

        &mut self.buckets[j][k]
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = SqrtBucket::new();

        s.insert(0, 1);

        assert_eq!(s.size(), 1);

        // dbg!(&s.buckets, s.node_of(1));
        s.insert(1, 2);

        // dbg!(&s.buckets, s.node_of(1));
        assert_eq!(s.size(), 2);

        s.remove(0);

        assert_eq!(s.size(), 1);

        // dbg!(&s.buckets, s.node_of(0));
        assert!(s[0] == 2);

        s.remove(0);

        s.insert(0, 1);

        assert_eq!(s.size(), 1);

        // let m = 0;
        for i in 0..1 << 10 {
            // if i % 100 == 0 {
            //     println!("{}", i);
            // }
            s.insert(s.size(), i);
            // if s.buckets.len() != m{
            //     println!("{:?}", s.buckets);
            //     m = s.buckets.len();
            // }
        }
    }
}
