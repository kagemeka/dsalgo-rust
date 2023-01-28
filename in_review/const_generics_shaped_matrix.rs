use std::ops::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]

pub struct Matrix<T, const H: usize, const W: usize>([[T; W]; H]);

impl<T: Copy, const H: usize, const W: usize> Matrix<T, H, W> {
    pub fn new(fill_value: T) -> Self { Self([[fill_value; W]; H]) }
}

impl<T: Copy, const H: usize, const W: usize> Default for Matrix<T, H, W>
where
    T: Default,
{
    fn default() -> Self { Self::new(T::default()) }
}

impl<T: Copy, const H: usize, const W: usize> From<[[T; W]; H]>
    for Matrix<T, H, W>
{
    fn from(a: [[T; W]; H]) -> Self { Self(a) }
}

impl<T, const H: usize, const W: usize> Index<usize> for Matrix<T, H, W> {
    type Output = [T; W];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<T, const H: usize, const W: usize> IndexMut<usize> for Matrix<T, H, W> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T, const H: usize, const W: usize> std::fmt::Display for Matrix<T, H, W>
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
        type Mat = Matrix<i64, 4, 3>;

        let mut a = Mat::new(0);

        a[1][1] += 1;

        println!("{:?}", a);

        println!("{}", a);
    }
}
