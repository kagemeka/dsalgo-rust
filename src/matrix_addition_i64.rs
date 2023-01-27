pub fn add(
    a: &[Vec<i64>],
    b: &[Vec<i64>],
) -> Vec<Vec<i64>> {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.iter().zip(b.iter()).map(|(a, b)| a + b).collect())
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![vec![0, 1], vec![2, 3], vec![4, 5]];

        let c = add(&a, &a);

        assert_eq!(c, [[0, 2], [4, 6], [8, 10]]);
    }
}
