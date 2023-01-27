use crate::dynamic_shaped_matrix::Matrix;

impl<T: Clone> Matrix<T> {
    pub fn transpose(&self) -> Self {
        let (h, w) = self.shape();

        let mut a = vec![vec![None; h]; w];

        for i in 0..h {
            for j in 0..w {
                a[j][i] = Some(self[i][j].clone());
            }
        }

        a.into_iter()
            .map(|row| row.into_iter().map(|x| x.unwrap()).collect())
            .collect::<Vec<_>>()
            .into()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Mat = Matrix<usize>;

        let a: Mat = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]].into();

        assert_eq!(
            a.transpose(),
            [[0, 4, 8], [1, 5, 9], [2, 6, 10], [3, 7, 11]].into()
        );
    }
}
