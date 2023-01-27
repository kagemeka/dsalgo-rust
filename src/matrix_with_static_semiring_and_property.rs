use std::{
    marker::PhantomData,
    ops::*,
};

pub trait Semiring {
    type T;

    fn add(
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn zero() -> Self::T;

    fn mul(
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn one() -> Self::T;
}

use crate::static_matrix_property_trait::Shape;

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Matrix<R: Semiring, P>(pub Vec<Vec<R::T>>, PhantomData<P>);

impl<R: Semiring, P: Shape> Matrix<R, P>
where
    R::T: Clone,
{
    pub fn new(fill_value: R::T) -> Self {
        let (h, w) = P::shape();

        Self(vec![vec![fill_value; w]; h], PhantomData)
    }
}

impl<R: Semiring, P: Shape> Default for Matrix<R, P>
where
    R::T: Clone + Default,
{
    fn default() -> Self { Self::new(R::T::default()) }
}

impl<R: Semiring, P> Index<usize> for Matrix<R, P> {
    type Output = [R::T];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<R: Semiring, P> IndexMut<usize> for Matrix<R, P> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<R: Semiring, P: Shape> From<Vec<Vec<R::T>>> for Matrix<R, P> {
    fn from(data: Vec<Vec<R::T>>) -> Self {
        let (h, w) = P::shape();

        assert_eq!(h, data.len());

        for i in 0..h {
            assert_eq!(data[i].len(), w);
        }

        Self(data, PhantomData)
    }
}

impl<R: Semiring, P: Shape, const H: usize, const W: usize> From<[[R::T; W]; H]>
    for Matrix<R, P>
where
    R::T: Clone,
{
    fn from(data: [[R::T; W]; H]) -> Self {
        let (h, w) = P::shape();

        assert!(H == h && W == w);

        Self(data.into_iter().map(|x| x.to_vec()).collect(), PhantomData)
    }
}

use crate::static_square_matrix_property_trait::Size;

impl<R: Semiring, P> Add for Matrix<R, P>
where
    P: Shape,
    R::T: AddAssign + Clone,
{
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        let (h, w) = P::shape();

        for i in 0..h {
            for j in 0..w {
                self[i][j] += rhs[i][j].clone();
            }
        }

        self
    }
}

impl<R: Semiring, P> AddAssign for Matrix<R, P>
where
    P: Shape + Clone,
    R: Clone,
    R::T: AddAssign + Clone,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone() + rhs;
    }
}

impl<R: Semiring, P> From<i32> for Matrix<R, P>
where
    P: Size,
    R::T: Mul<Output = R::T> + AddAssign + Clone,
{
    fn from(x: i32) -> Self {
        assert!(x == 0 || x == 1);

        let n = P::size();

        let mut a = Self::new(R::zero());

        if x == 1 {
            for i in 0..n {
                a[i][i] = R::one();
            }
        }

        a
    }
}

impl<R: Semiring, P> Mul for Matrix<R, P>
where
    P: Size,
    R::T: Mul<Output = R::T> + AddAssign + Clone,
{
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        let n = P::size();

        let mut a: Self = 0.into();

        for i in 0..n {
            for k in 0..n {
                for j in 0..n {
                    a[i][j] += self[i][k].clone() * rhs[k][j].clone();
                }
            }
        }

        a
    }
}

impl<R: Semiring, P> MulAssign for Matrix<R, P>
where
    P: Size + Clone,
    R: Clone,
    R::T: Mul<Output = R::T> + AddAssign + Clone,
{
    fn mul_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        #[derive(Clone, Debug, Eq, PartialEq)]

        struct P;

        impl Size for P {
            fn size() -> usize { 3 }
        }

        #[derive(Clone, Debug, Eq, PartialEq)]

        struct R;

        impl Semiring for R {
            type T = i64;

            fn add(
                l: Self::T,
                r: Self::T,
            ) -> Self::T {
                l + r
            }

            fn mul(
                l: Self::T,
                r: Self::T,
            ) -> Self::T {
                l * r
            }

            fn zero() -> Self::T { 0 }

            fn one() -> Self::T { 1 }
        }

        type Mat = Matrix<R, P>;

        let e: Mat = 1.into();

        let b: Mat = [[0, 1, 2], [3, 4, 5], [6, 7, 8]].into();

        assert_eq!(e.clone() * b.clone(), b);

        dbg!(b);
    }
}
