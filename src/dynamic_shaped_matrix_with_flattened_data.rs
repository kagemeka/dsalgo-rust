#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Matrix<T>(Vec<T>, (usize, usize));

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        let w = self.shape().1;

        &self.0[index.0 * w + index.1]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output {
        let w = self.shape().1;

        &mut self.0[index.0 * w + index.1]
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(
        h: usize,
        w: usize,
        fill_value: T,
    ) -> Matrix<T> {
        Matrix(vec![fill_value; h * w], (h, w))
    }

    pub fn to_2d_vec(&self) -> Vec<Vec<T>> {
        let w = self.shape().1;

        self.0.chunks(w).map(|x| x.to_vec()).collect()
    }
}

impl<T> Matrix<T> {
    pub fn shape(&self) -> (usize, usize) { self.1 }
}

impl<T: std::fmt::Debug> std::fmt::Display for Matrix<T> {
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

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let h = data.len();

        let w;

        if h > 0 {
            w = data[0].len();

            for i in 1..h {
                assert_eq!(data[i].len(), w);
            }
        } else {
            w = 0;
        }

        Self(data.into_iter().flatten().collect(), (h, w))
    }
}

impl<T, const H: usize, const W: usize> From<[[T; W]; H]> for Matrix<T> {
    fn from(data: [[T; W]; H]) -> Self {
        Self(data.into_iter().flatten().collect(), (H, W))
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Mat = Matrix<usize>;

        let mut a: Mat = [[0, 1], [2, 3]].into();

        assert_eq!(a.to_2d_vec(), vec![vec![0, 1], vec![2, 3]]);

        a[(0, 0)] += 1;

        assert_eq!(a.to_2d_vec(), vec![vec![1, 1], vec![2, 3]]);
    }
}
