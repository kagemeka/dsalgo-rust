#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(
        &mut self,
        i: usize,
    ) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(
        height: usize,
        width: usize,
        fill_value: T,
    ) -> Matrix<T> {
        Matrix(vec![vec![fill_value; width]; height])
    }

    pub fn shape(&self) -> (usize, usize) {
        if self.0.len() == 0 {
            (0, 0)
        } else {
            (self.0.len(), self.0[0].len())
        }
    }
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

        if h > 0 {
            let w = data[0].len();

            for i in 1..h {
                assert_eq!(data[i].len(), w);
            }
        }

        Self(data)
    }
}

impl<T: Clone, const H: usize, const W: usize> From<[[T; W]; H]> for Matrix<T> {
    fn from(data: [[T; W]; H]) -> Self {
        Self(data.into_iter().map(|x| x.to_vec()).collect())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let (height, width) = (3, 4);

        type Mat = Matrix<usize>;

        let mut a = Mat::new(height, width, 0);

        assert_eq!(a.0, [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

        println!("{}", a);

        a[1][1] += 1;

        assert_eq!(a, [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]].into());

        for i in 0..height {
            for j in 0..width {
                a[i][j] = i * width + j;
            }
        }

        assert_eq!(a.0, [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]]);
    }
}
