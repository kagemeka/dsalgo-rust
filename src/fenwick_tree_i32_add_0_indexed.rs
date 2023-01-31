pub struct Fenwick(Vec<i32>);

impl Fenwick {
    pub fn new(size: usize) -> Self { Self(vec![0; size]) }

    pub fn size(&self) -> usize { self.0.len() }

    pub fn add(
        &mut self,
        mut i: usize,
        x: i32,
    ) {
        let n = self.size();

        assert!(i < n);

        while i < n {
            self.0[i] += x;

            i += i + 1;
        }
    }

    pub fn sum_le(
        &self,
        mut i: usize,
    ) -> i32 {
        assert!(i < self.size());

        let mut v = 0;

        loop {
            v += self.0[i];

            i = i & (i + 1);

            if i == 0 {
                return v;
            }

            i -= 1;
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut fw = Fenwick::new(10);

        fw.add(5, 1);

        assert_eq!(fw.sum_le(4), 0);

        assert_eq!(fw.sum_le(5), 1);
    }
}
