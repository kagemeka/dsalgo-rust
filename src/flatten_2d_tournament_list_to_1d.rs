pub fn flatten_tournament(result: &[Vec<usize>]) -> Vec<usize> {
    let mut a = vec![];

    for level_res in result.iter() {
        a.append(&mut level_res.to_vec());
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let res = vec![vec![1, 2, 3, 4], vec![2, 3], vec![3]];

        assert_eq!(flatten_tournament(&res), [1, 2, 3, 4, 2, 3, 3]);
    }
}
