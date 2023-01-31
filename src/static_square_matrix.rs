use std::{
    ops::*,
    sync::atomic::{
        AtomicUsize,
        Ordering::SeqCst,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> {
    fn cell() -> &'static AtomicUsize {
        static CELL: AtomicUsize = AtomicUsize::new(0);

        &CELL
    }

    pub fn size() -> usize { Self::cell().load(SeqCst) }

    pub fn set_size(size: usize) { Self::cell().store(size, SeqCst); }
}

impl<T: Clone> Matrix<T> {
    pub fn new(fill_value: T) -> Self {
        let n = Self::size();

        Self(vec![vec![fill_value; n]; n])
    }
}

impl<T: Clone + Default> Default for Matrix<T> {
    fn default() -> Self { Self::new(T::default()) }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let n = Self::size();

        assert_eq!(n, data.len());

        for i in 0..n {
            assert_eq!(data[i].len(), n);
        }

        Self(data)
    }
}

impl<T: Clone, const N: usize> From<[[T; N]; N]> for Matrix<T> {
    fn from(data: [[T; N]; N]) -> Self {
        let n = Self::size();

        assert_eq!(N, n);

        Self(data.iter().map(|x| x.to_vec()).collect())
    }
}

impl<T: AddAssign + Clone + From<i32>> Add for Matrix<T> {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        let n = Self::size();

        for i in 0..n {
            for j in 0..n {
                self[i][j] += rhs[i][j].clone();
            }
        }

        self
    }
}

impl<T: AddAssign + Clone + From<i32>> AddAssign for Matrix<T> {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone() + rhs;
    }
}

impl<T: Mul<Output = T> + AddAssign + Clone + From<i32>> From<i32>
    for Matrix<T>
{
    fn from(x: i32) -> Self {
        assert!(x == 0 || x == 1);

        let n = Self::size();

        let mut a = Self::new(0.into());

        if x == 1 {
            for i in 0..n {
                a[i][i] = 1.into();
            }
        }

        a
    }
}

impl<T: Mul<Output = T> + AddAssign + Clone + From<i32>> Mul for Matrix<T> {
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        let n = Self::size();

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

impl<T: Mul<Output = T> + AddAssign + Clone + From<i32>> MulAssign
    for Matrix<T>
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
        type Mat = Matrix<i64>;

        Mat::set_size(3);

        let e: Mat = 1.into();

        let b: Mat = [[0, 1, 2], [3, 4, 5], [6, 7, 8]].into();

        assert_eq!(e.clone() * b.clone(), b);

        dbg!(b);
    }
}
