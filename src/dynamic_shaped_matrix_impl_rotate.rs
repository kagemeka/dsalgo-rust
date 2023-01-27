use crate::dynamic_shaped_matrix::Matrix;

impl<T: Default + Clone> Matrix<T> {
    pub fn rot90(&self) -> Self { self.transpose().reverse() }

    pub fn rot270(&self) -> Self { self.reverse().transpose() }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Mat = Matrix<usize>;

        let a: Mat = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]].into();

        assert_eq!(
            a.rot90(),
            [[3, 7, 11], [2, 6, 10], [1, 5, 9], [0, 4, 8]].into()
        );

        assert_eq!(
            a.rot270(),
            [[8, 4, 0], [9, 5, 1], [10, 6, 2], [11, 7, 3]].into()
        );
    }
}
