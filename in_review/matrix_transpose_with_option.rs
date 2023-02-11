pub fn transpose<T: Clone>(a: &[Vec<T>]) -> Vec<Vec<T>> {
    let h = a.len();

    if h == 0 {
        return vec![];
    }

    let w = a[0].len();

    let mut t: Vec<Vec<Option<T>>> = vec![vec![None; h]; w];

    for i in 0..h {
        for j in 0..w {
            t[j][i] = Some(a[i][j].clone());
        }
    }

    t.into_iter()
        .map(|row| row.into_iter().map(|x| x.unwrap()).collect())
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![vec![0, 1, 2], vec![3, 4, 5]];

        assert_eq!(transpose(&a), vec![vec![0, 3], vec![1, 4], vec![2, 5]]);
    }
}
