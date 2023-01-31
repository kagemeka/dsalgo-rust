use crate::sqrt_decomposition_i64_add::SqrtDecomposition;

pub struct Multiset(SqrtDecomposition);

impl Multiset {
    pub fn new(less_than: usize) -> Self {
        Self(SqrtDecomposition::new(less_than))
    }

    pub fn size(&self) -> usize { self.0.fold(0, self.0.size()) as usize }

    pub fn count(
        &self,
        x: usize,
    ) -> i64 {
        self.0.fold(x, x + 1)
    }

    /// positive cnt: insert.
    /// negative cnt: remove. (|cnt| <= count(x))

    pub fn add(
        &mut self,
        x: usize,
        delta: i64,
    ) {
        assert!(self.count(x) + delta >= 0);

        self.0.set(x, self.0[x] + delta);
    }

    pub fn insert(
        &mut self,
        x: usize,
    ) {
        self.add(x, 1);
    }

    pub fn remove(
        &mut self,
        x: usize,
    ) {
        self.add(x, -1);
    }

    pub fn remove_all(
        &mut self,
        x: usize,
    ) {
        self.add(x, -self.count(x));
    }

    pub fn lower_bound(
        &self,
        x: usize,
    ) -> usize {
        self.0.fold(0, x) as usize
    }

    pub fn upper_bound(
        &self,
        x: usize,
    ) -> usize {
        self.0.fold(0, x + 1) as usize
    }

    pub fn get(
        &self,
        i: usize,
    ) -> Option<usize> {
        let v = self.0.max_right(|&x| x as usize <= i, 0);

        if v == self.0.size() {
            None
        } else {
            Some(v)
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = Multiset::new(10);

        s.add(5, 2);

        assert_eq!(s.size(), 2);

        assert_eq!(s.count(5), 2);

        assert_eq!(s.count(0), 0);

        assert_eq!(s.lower_bound(5), 0);

        assert_eq!(s.upper_bound(5), 2);

        assert_eq!(s.get(0), Some(5));

        assert_eq!(s.get(1), Some(5));

        s.add(5, -2);

        assert_eq!(s.size(), 0);
    }
}
