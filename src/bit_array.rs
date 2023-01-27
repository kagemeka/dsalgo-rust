// TODO: currently, this is implemented as dynamic sized.
// but bit array should have static size.
const K: usize = 6; // chunk size = 2^K
const M: usize = (1 << K) - 1;

fn bucket(index: usize) -> usize { index >> K }

fn point(index: usize) -> usize { index & M }

fn value(index: usize) -> usize { 1 << point(index) }

#[derive(Clone)]

pub struct BitArray(Vec<usize>);

impl BitArray {
    pub fn new(size: usize) -> Self { BitArray(vec![0; (size + M) >> K]) }

    pub fn set(
        &mut self,
        i: usize,
    ) {
        self.0[bucket(i)] |= value(i);
    }

    pub fn reset(
        &mut self,
        i: usize,
    ) {
        self.0[bucket(i)] &= !value(i);
    }

    pub fn flip(
        &mut self,
        i: usize,
    ) {
        self.0[bucket(i)] ^= value(i);
    }

    pub fn is_set(
        &self,
        i: usize,
    ) -> bool {
        self.0[bucket(i)] >> point(i) & 1 == 1
    }
}

impl From<&[bool]> for BitArray {
    fn from(is_set: &[bool]) -> Self {
        let mut a = BitArray::new(is_set.len());

        for (i, &ok) in is_set.iter().enumerate() {
            if ok {
                a.set(i)
            }
        }

        a
    }
}

use std::ops::*;

fn min_len(
    lhs: &BitArray,
    rhs: &BitArray,
) -> usize {
    lhs.0.len().min(rhs.0.len())
}

fn max_len(
    lhs: &BitArray,
    rhs: &BitArray,
) -> usize {
    lhs.0.len().max(rhs.0.len())
}

impl BitAnd for BitArray {
    type Output = Self;

    fn bitand(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        for i in 0..min_len(&self, &rhs) {
            self.0[i] &= rhs.0[i];
        }

        self
    }
}

impl BitArray {
    pub fn popcount(&self) -> u64 {
        self.0.iter().map(|x| x.count_ones() as u64).sum()
    }
}

// TODO:
// &, |,  <<, >>, ==
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
