use crate::integer_square_root_with_binary_search_usize::isqrt;

pub struct SqrtDecomposition {
    data: Vec<i64>,
    buckets: Vec<i64>,
}

impl SqrtDecomposition {
    pub fn size(&self) -> usize { self.data.len() }

    pub fn interval(&self) -> usize {
        let n = self.buckets.len();

        (self.size() + n - 1) / n
    }

    pub fn new(size: usize) -> Self {
        let data = vec![0; size];

        let m = isqrt(size);

        let buckets = vec![0; (size + m - 1) / m];

        Self { data, buckets }
    }

    fn merge(
        &mut self,
        j: usize,
    ) {
        let n = self.interval();

        self.buckets[j] = self.data[j * n..self.size().min((j + 1) * n)]
            .iter()
            .cloned()
            .fold(0, |x, y| x + y)
    }

    pub fn set(
        &mut self,
        i: usize,
        x: i64,
    ) {
        self.data[i] = x;

        self.merge(i / self.interval());
    }

    pub fn fold(
        &self,
        l: usize,
        r: usize,
    ) -> i64 {
        assert!(l <= r && r <= self.size());

        let n = self.interval();

        let mut v = 0;

        let lj = (l + n - 1) / n;

        let rj = r / n;

        if rj < lj {
            for x in self.data[l..r].iter() {
                v += x;
            }

            return v;
        }

        for x in self.data[l..lj * n].iter() {
            v += x;
        }

        for x in self.buckets[lj..rj].iter() {
            v += x;
        }

        for x in self.data[rj * n..r].iter() {
            v += x;
        }

        v
    }

    pub fn max_right<F>(
        &self,
        is_ok: F,
        l: usize,
    ) -> usize
    where
        F: Fn(&i64) -> bool,
    {
        let m = self.interval();

        let n = self.size();

        let lj = (l + m - 1) / m;

        let mut v = 0;

        for i in l..lj * m {
            let nv = v + self.data[i];

            if !is_ok(&nv) {
                return i;
            }

            v = nv;
        }

        let mut i = n;

        for j in lj..self.buckets.len() {
            let nv = v + self.buckets[j];

            if !is_ok(&nv) {
                i = j * m;

                break;
            }

            v = nv;
        }

        while i < n {
            let nv = v + self.data[i];

            if !is_ok(&nv) {
                return i;
            }

            v = nv;

            i += 1;
        }

        i
    }

    pub fn min_left<F>(
        &self,
        is_ok: F,
        r: usize,
    ) -> usize
    where
        F: Fn(&i64) -> bool,
    {
        let m = self.interval();

        let rj = r / m;

        let mut v = 0;

        for i in (rj * m..r).rev() {
            let nv = self.data[i] + v;

            if !is_ok(&nv) {
                return i + 1;
            }

            v = nv;
        }

        let mut i = 0;

        for j in (0..rj).rev() {
            let nv = self.buckets[j] + v;

            if !is_ok(&nv) {
                i = (j + 1) * m;

                break;
            }

            v = nv;
        }

        while i > 0 {
            i -= 1;

            let nv = self.data[i] + v;

            if !is_ok(&nv) {
                return i + 1;
            }

            v = nv;
        }

        i
    }
}

use std::ops::*;

impl Index<usize> for SqrtDecomposition {
    type Output = i64;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.data[i]
    }
}

impl From<&[i64]> for SqrtDecomposition {
    fn from(data: &[i64]) -> Self {
        let mut sqd = SqrtDecomposition::new(data.len());

        sqd.data.clone_from_slice(data);

        for j in 0..sqd.buckets.len() {
            sqd.merge(j);
        }

        sqd
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut sqd = SqrtDecomposition::new(5);

        for i in 0..5 {
            sqd.set(i, i as i64 + 1);
        }

        assert_eq!(sqd[3], 4);

        assert_eq!(sqd.fold(2, 4), 7);

        assert_eq!(sqd.max_right(|&x| x < 5, 1), 2);

        assert_eq!(sqd.max_right(|&x| x <= 5, 1), 3);

        assert_eq!(sqd.min_left(|&x| x < 7, 4), 3);

        assert_eq!(sqd.min_left(|&x| x <= 7, 4), 2);
    }
}
