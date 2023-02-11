pub fn transpose<T: Clone>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let h = a.len();

    if h == 0 {
        return vec![];
    }

    let w = a[0].len();

    let mut t = vec![Vec::with_capacity(h); w];

    for row in a.into_iter() {
        for (j, x) in row.into_iter().enumerate() {
            t[j].push(x);
        }
    }

    t
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![vec![0, 1, 2], vec![3, 4, 5]];

        assert_eq!(transpose(a), vec![vec![0, 3], vec![1, 4], vec![2, 5]]);
    }
}
