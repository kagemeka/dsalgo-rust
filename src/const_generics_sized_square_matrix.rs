use std::ops::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]

pub struct Matrix<T, const N: usize>([[T; N]; N]);

impl<T: Copy, const N: usize> Matrix<T, N> {
    pub fn new(fill_value: T) -> Self { Self([[fill_value; N]; N]) }
}

impl<T: Copy, const N: usize> Default for Matrix<T, N>
where
    T: Default,
{
    fn default() -> Self { Self::new(T::default()) }
}

impl<T: Copy, const N: usize> From<[[T; N]; N]> for Matrix<T, N> {
    fn from(a: [[T; N]; N]) -> Self { Self(a) }
}

impl<T, const N: usize> Index<usize> for Matrix<T, N> {
    type Output = [T; N];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<T, const N: usize> IndexMut<usize> for Matrix<T, N> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T, const N: usize> std::fmt::Display for Matrix<T, N>
where
    T: std::fmt::Debug + Copy,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let format_str = self
            .0
            .iter()
            .map(|row| format!("{:?}", row))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", format_str)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Mat = Matrix<i64, 4>;

        let mut a = Mat::new(0);

        a[1][1] += 1;

        println!("{:?}", a);

        println!("{}", a);
    }
}
