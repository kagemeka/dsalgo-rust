pub fn argsort(p: &[usize]) -> Vec<usize> {
    let mut idx = vec![0; p.len()];

    for (i, &j) in p.iter().enumerate() {
        idx[j] = i;
    }

    idx
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![3, 0, 1, 2];

        assert_eq!(argsort(&a), [1, 2, 3, 0]);
    }
}
