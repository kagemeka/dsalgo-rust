pub fn argsort<T: Ord>(a: &[T]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..a.len()).collect();

    idx.sort_by_key(|&i| &a[i]);

    idx
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![3, -1, 0, 5];

        assert_eq!(argsort(&a), [1, 2, 0, 3]);
    }
}
