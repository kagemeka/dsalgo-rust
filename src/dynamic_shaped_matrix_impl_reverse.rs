use crate::dynamic_shaped_matrix::Matrix;

impl<T: Default + Clone> Matrix<T> {
    pub fn reverse(&self) -> Self {
        let mut a = Self(self.0.clone());

        a.0.reverse();

        a
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
            a.reverse(),
            [[8, 9, 10, 11], [4, 5, 6, 7], [0, 1, 2, 3]].into()
        );
    }
}
