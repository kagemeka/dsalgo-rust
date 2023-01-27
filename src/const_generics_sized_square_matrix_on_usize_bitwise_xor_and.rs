use std::ops::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]

pub struct Matrix<const N: usize>([[usize; N]; N]);

impl<const N: usize> Matrix<N> {
    pub fn new(fill_value: usize) -> Self { Self([[fill_value; N]; N]) }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [usize; N];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<const N: usize> Add for Matrix<N> {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        for i in 0..N {
            for j in 0..N {
                self[i][j] ^= rhs[i][j];
            }
        }

        self
    }
}

impl<const N: usize> AddAssign for Matrix<N> {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self + rhs;
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut a: Self = 0.into();

        for i in 0..N {
            for k in 0..N {
                for j in 0..N {
                    a[i][j] ^= self[i][k] & rhs[k][j];
                }
            }
        }

        a
    }
}

impl<const N: usize> MulAssign for Matrix<N> {
    fn mul_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self * rhs;
    }
}

impl<const N: usize> From<i32> for Matrix<N> {
    fn from(x: i32) -> Self {
        assert!(x == 0 || x == 1);

        let mut a = Self::new(0);

        if x == 1 {
            for i in 0..N {
                a[i][i] = std::usize::MAX;
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
        type Mat = Matrix<3>;

        let mut a = Mat::new(0);

        a[1][1] ^= 1;

        println!("{:?}", a);

        let a: Mat = 1.into();

        let b = Matrix::<3>([[2, 3, 4], [5, 6, 7], [8, 9, 10]]);

        let c = a * b;

        assert_eq!(c, b);

        let zero = 0.into();

        assert_eq!(b * zero, zero);
    }
}
