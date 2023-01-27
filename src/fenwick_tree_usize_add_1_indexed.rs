pub struct Fenwick(Vec<usize>);

impl Fenwick {
    pub fn new(size: usize) -> Self { Self(vec![0; size + 1]) }

    pub fn size(&self) -> usize { self.0.len() - 1 }

    pub fn add(
        &mut self,
        mut i: usize,
        x: usize,
    ) {
        i += 1;

        while i <= self.size() {
            self.0[i] += x;

            i += 1 << i.trailing_zeros();
        }
    }

    pub fn get(
        &self,
        mut i: usize,
    ) -> usize {
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

        assert_eq!(fw.get(5), 0);

        assert_eq!(fw.get(6), 1);
    }
}
