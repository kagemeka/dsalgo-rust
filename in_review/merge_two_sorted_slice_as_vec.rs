pub fn merge_sorted<T: Ord + Clone>(
    a: &[T],
    b: &[T],
) -> Vec<T> {
    let n = a.len();

    let m = b.len();

    let mut i = 0;

    let mut j = 0;

    let mut res = Vec::with_capacity(n + m);

    while i < n && j < m {
        if a[i] <= b[j] {
            res.push(a[i].clone());

            i += 1;
        } else {
            res.push(b[j].clone());

            j += 1;
        }
    }

    res.extend(a[i..].to_owned().into_iter());

    res.extend(b[j..].to_owned().into_iter());

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (vec![2, 4, 6], vec![1, 3, 3, 7]),
            vec![1, 2, 3, 3, 4, 6, 7],
        )];

        for ((a, b), ans) in cases {
            assert_eq!(merge_sorted(&a, &b), ans);
        }
    }
}
