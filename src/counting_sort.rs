use crate::counting_argsort::argsort;

pub fn counting_sort(a: &[usize]) -> Vec<usize> {
    argsort(a.to_vec()).into_iter().map(|i| a[i]).collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![3, 3, 0, 2, 1];

        assert_eq!(counting_sort(&a), [0, 1, 2, 3, 3]);

        let a = vec![4, 3, 1, 6, 0, 6, 3];

        let mut b = a.clone();

        b.sort();

        assert_eq!(counting_sort(&a), b);
    }
}
