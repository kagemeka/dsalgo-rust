pub fn unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    a.sort_unstable();

    a.dedup();

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![-1, 3, 0, 5, 3, 0];

        assert_eq!(unique(a), [-1, 0, 3, 5]);
    }
}
