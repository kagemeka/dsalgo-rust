use std::ops::*;

pub struct Fenwick<T>(Vec<T>);

impl<T> Fenwick<T> {
    pub fn size(&self) -> usize { self.0.len() - 1 }

    pub fn new(size: usize) -> Self
    where
        T: From<i32>,
    {
        Self((0..size + 1).map(|_| 0.into()).collect())
    }

    pub fn operate(
        &mut self,
        mut i: usize,
        x: T,
    ) where
        T: BitXorAssign + Clone,
    {
        i += 1;

        while i <= self.size() {
            self.0[i] ^= x.clone();

            i += 1 << i.trailing_zeros();
        }
    }

    pub fn get(
        &self,
        mut i: usize,
    ) -> T
    where
        T: From<i32> + BitXorAssign + Clone,
    {
        let mut v = 0.into();

        while i > 0 {
            v ^= self.0[i].clone();

            i -= 1 << i.trailing_zeros();
        }

        v
    }

    pub fn max_right<F: Fn(&T) -> bool>(
        &self,
        f: F,
    ) -> usize
    where
        T: From<i32> + BitXor<Output = T> + Clone,
    {
        let n = self.size();

        let mut d = (n + 1).next_power_of_two();

        let mut v: T = 0.into();

        let mut i = 0;

        loop {
            d >>= 1;

            if d == 0 {
                return i;
            }

            if i + d > n {
                continue;
            }

            let nv = v.clone() ^ self.0[i + d].clone();

            if f(&nv) {
                v = nv;

                i += d;
            }
        }
    }
}
