//! interface is 0-indexed.

pub struct Fenwick(Vec<i64>);

impl Fenwick {
    pub fn new(size: usize) -> Self { Self(vec![0; size + 1]) }

    pub fn size(&self) -> usize { self.0.len() - 1 }

    /// add x to a_j (j >= i)

    pub fn add_ge(
        &mut self,
        mut i: usize,
        x: i64,
    ) {
        assert!(i < self.size());

        i += 1;

        while i <= self.size() {
            self.0[i] += x;

            i += 1 << i.trailing_zeros();
        }
    }

    /// get a_i

    pub fn get(
        &self,
        mut i: usize,
    ) -> i64 {
        assert!(i < self.size());

        i += 1;

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

        fw.add_ge(5, 1); // add 1 for a_j (j >= i);
        assert_eq!(fw.get(4), 0); // get a_4
        assert_eq!(fw.get(5), 1);
    }
}
