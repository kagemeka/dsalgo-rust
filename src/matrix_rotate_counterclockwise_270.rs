use crate::matrix_transpose::transpose;

pub fn rot270<T: Clone>(mut a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    a.reverse();

    transpose(a)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a = vec![vec![0, 1], vec![2, 3]];

        a = rot270(a);

        assert_eq!(a, vec![vec![2, 0], vec![3, 1]]);
    }
}
