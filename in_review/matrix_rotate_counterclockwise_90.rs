use crate::matrix_transpose::transpose;

pub fn rot90<T: Clone>(mut a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    a = transpose(a);

    a.reverse();

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a = vec![vec![0, 1], vec![2, 3]];

        a = rot90(a);

        assert_eq!(a, vec![vec![1, 3], vec![0, 2]]);
    }
}
