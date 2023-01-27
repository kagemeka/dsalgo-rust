pub struct ArrayCompression<T>(pub Vec<T>);

impl<T: Ord> ArrayCompression<T> {
    pub fn new(mut a: Vec<T>) -> Self {
        a.sort_unstable();

        a.dedup();

        Self(a)
    }

    pub fn encode(
        &self,
        v: &T,
    ) -> usize {
        self.0.binary_search(v).unwrap()
    }

    pub fn once(a: &[T]) -> Vec<usize>
    where
        T: Clone,
    {
        let f = Self::new(a.to_vec());

        a.iter().map(|x| f.encode(x)).collect()
    }
}

use std::ops::*;

impl<T: Ord> Index<usize> for ArrayCompression<T> {
    type Output = T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let arr = [4, 3, 0, -1, 3, 10];

        let f = ArrayCompression::new(arr.to_vec());

        assert_eq!(f.encode(&-1), 0);

        assert_eq!(f.encode(&10), 4);

        assert_eq!(f[0], -1);

        // f.encode(&5); // error
        assert_eq!(ArrayCompression::once(&arr), vec![3, 2, 1, 0, 2, 4]);
    }
}
