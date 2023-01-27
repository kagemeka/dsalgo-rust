#[derive(Debug, Clone, Copy, Eq, PartialEq)]

pub struct Matrix<T, const H: usize, const W: usize>([T; H * W])
where
    [(); H * W]:;

impl<T, const H: usize, const W: usize> Matrix<T, H, W>
where
    [(); H * W]:,
    T: Copy,
{
    pub fn new(fill_value: T) -> Self { Self([fill_value; H * W]) }
}

impl<T, const H: usize, const W: usize> Default for Matrix<T, H, W>
where
    [(); H * W]:,
    T: Copy + Default,
{
    fn default() -> Self { Self::new(T::default()) }
}

impl<T, const H: usize, const W: usize> From<[[T; W]; H]> for Matrix<T, H, W>
where
    [(); H * W]:,
    T: Copy + std::fmt::Debug,
{
    fn from(a: [[T; W]; H]) -> Self {
        Self(a.into_iter().flatten().collect::<Vec<_>>().try_into().unwrap())
    }
}

use std::ops::*;

impl<T, const H: usize, const W: usize> Index<(usize, usize)>
    for Matrix<T, H, W>
where
    [(); H * W]:,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &self.0[index.0 * W + index.1]
    }
}

impl<T, const H: usize, const W: usize> IndexMut<(usize, usize)>
    for Matrix<T, H, W>
where
    [(); H * W]:,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output {
        &mut self.0[index.0 * W + index.1]
    }
}

impl<T, const H: usize, const W: usize> Matrix<T, H, W>
where
    [(); H * W]:,
    T: Copy + std::fmt::Debug,
{
    fn to_2d_array(&self) -> [[T; W]; H] {
        self.0
            .array_chunks::<W>()
            .map(|x| *x)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

impl<T, const H: usize, const W: usize> std::fmt::Display for Matrix<T, H, W>
where
    [(); H * W]:,
    T: std::fmt::Debug + Copy,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let format_str = self
            .to_2d_array()
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
        type Mat = Matrix<i64, 3, 2>;

        let mut a = Mat::default();

        a[(1, 1)] += 1;

        println!("{:?}", a);

        println!("{}", a);

        a = [[1, 2], [3, 4], [5, 6]].into();

        println!("{}", a);
    }
}
