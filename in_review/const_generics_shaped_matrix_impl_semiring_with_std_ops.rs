use std::ops::*;

use crate::const_generics_shaped_matrix::Matrix;

impl<T, const H: usize, const W: usize> Add for Matrix<T, H, W>
where
    T: Clone + AddAssign,
{
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        for i in 0..H {
            for j in 0..W {
                self[i][j] += rhs[i][j].clone();
            }
        }

        self
    }
}

impl<T, const H: usize, const W: usize> AddAssign for Matrix<T, H, W>
where
    T: Clone + AddAssign,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone() + rhs;
    }
}

impl<T, const H: usize, const K: usize, const W: usize> Mul<Matrix<T, K, W>>
    for Matrix<T, H, K>
where
    T: Copy + AddAssign + From<i32> + Mul<Output = T>,
{
    type Output = Matrix<T, H, W>;

    fn mul(
        self,
        rhs: Matrix<T, K, W>,
    ) -> Self::Output {
        let mut a = Matrix::<T, H, W>::new(0.into());

        for i in 0..H {
            for k in 0..K {
                for j in 0..W {
                    a[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        a
    }
}

impl<T, const N: usize> MulAssign for Matrix<T, N, N>
where
    T: Copy + AddAssign + From<i32> + Mul<Output = T>,
{
    fn mul_assign(
        &mut self,
        rhs: Matrix<T, N, N>,
    ) {
        *self = *self * rhs;
    }
}

impl<T, const N: usize> From<i32> for Matrix<T, N, N>
where
    T: From<i32> + Copy,
{
    fn from(x: i32) -> Self {
        assert!(x == 0 || x == 1);

        let mut a = Self::new(0.into());

        if x == 1 {
            for i in 0..N {
                a[i][i] = 1.into();
            }
        }

        a
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Mat = Matrix<i64, 3, 3>;

        let a: Mat = 1.into();

        let b: Mat = [[2, 3, 4], [5, 6, 7], [8, 9, 10]].into();

        let c = a * b;

        assert_eq!(c, b);

        let zero = 0.into();

        assert_eq!(b * zero, zero);
    }
}
