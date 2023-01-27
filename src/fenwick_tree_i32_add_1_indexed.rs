pub struct Fenwick(Vec<i32>);

impl Fenwick {
    pub fn new(size: usize) -> Self { Self(vec![0; size + 1]) }

    pub fn size(&self) -> usize { self.0.len() - 1 }

    pub fn add(
        &mut self,
        mut i: usize,
        x: i32,
    ) {
        let n = self.size();

        assert!(i < n);

        i += 1;

        while i <= n {
            self.0[i] += x;

            i += 1 << i.trailing_zeros();
        }
    }

    pub fn sum_lt(
        &self,
        mut i: usize,
    ) -> i32 {
        assert!(i <= self.size());

        let mut v = 0;

        while i > 0 {
            v += self.0[i];

            i -= 1 << i.trailing_zeros();
        }

        v
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut fw = Fenwick::new(10);

        fw.add(5, 1);

        assert_eq!(fw.sum_lt(5), 0);

        assert_eq!(fw.sum_lt(6), 1);
    }
}
