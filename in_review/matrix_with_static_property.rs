use std::{
    marker::PhantomData,
    ops::*,
};

use crate::static_matrix_property_trait::Shape;

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Matrix<T, P>(pub Vec<Vec<T>>, PhantomData<P>);

impl<T: Clone, P: Shape> Matrix<T, P> {
    pub fn new(fill_value: T) -> Self {
        let (h, w) = P::shape();

        Self(vec![vec![fill_value; w]; h], PhantomData)
    }
}

impl<T: Clone + Default, P: Shape> Default for Matrix<T, P> {
    fn default() -> Self { Self::new(T::default()) }
}

impl<T, P> Index<usize> for Matrix<T, P> {
    type Output = [T];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<T, P> IndexMut<usize> for Matrix<T, P> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T, P: Shape> From<Vec<Vec<T>>> for Matrix<T, P> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let (h, w) = P::shape();

        assert_eq!(h, data.len());

        for i in 0..h {
            assert_eq!(data[i].len(), w);
        }

        Self(data, PhantomData)
    }
}

impl<T: Clone, P: Shape, const H: usize, const W: usize> From<[[T; W]; H]>
    for Matrix<T, P>
{
    fn from(data: [[T; W]; H]) -> Self {
        let (h, w) = P::shape();

        assert!(H == h && W == w);

        Self(data.into_iter().map(|x| x.to_vec()).collect(), PhantomData)
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
