use std::ops::*;

use crate::{
    matrix_with_static_property::Matrix,
    static_matrix_property_trait::Shape,
    static_square_matrix_property_trait::Size,
};

impl<T, P> Add for Matrix<T, P>
where
    P: Shape,
    T: AddAssign + Clone,
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

impl<T, P> AddAssign for Matrix<T, P>
where
    P: Shape + Clone,
    T: AddAssign + Clone,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone() + rhs;
    }
}

impl<T, P> From<i32> for Matrix<T, P>
where
    P: Size,
    T: Mul<Output = T> + AddAssign + Clone + From<i32>,
{
    fn from(x: i32) -> Self {
        assert!(x == 0 || x == 1);

        let n = P::size();

        let mut a = Self::new(0.into());

        if x == 1 {
            for i in 0..n {
                a[i][i] = 1.into();
            }
        }

        a
    }
}

impl<T, P> Mul for Matrix<T, P>
where
    P: Size,
    T: Mul<Output = T> + AddAssign + Clone + From<i32>,
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

impl<T, P> MulAssign for Matrix<T, P>
where
    P: Size + Clone,
    T: Mul<Output = T> + AddAssign + Clone + From<i32>,
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

        type Mat = Matrix<i64, P>;

        let e: Mat = 1.into();

        let b: Mat = [[0, 1, 2], [3, 4, 5], [6, 7, 8]].into();

        assert_eq!(e.clone() * b.clone(), b);

        dbg!(b);
    }
}
