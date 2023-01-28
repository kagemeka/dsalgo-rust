pub fn flat_nonzero<T>(a: &[T]) -> Vec<usize>
where
    T: From<i32> + PartialEq,
{
    a.iter()
        .enumerate()
        .filter_map(|(i, x)| if x != &0.into() { Some(i) } else { None })
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![4, 0, 2, -1, 3];

        assert_eq!(flat_nonzero(&a), [0, 2, 3, 4]);
    }
}
