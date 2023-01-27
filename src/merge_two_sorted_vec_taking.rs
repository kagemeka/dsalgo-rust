pub fn merge_sorted<T: Ord>(
    a: Vec<T>,
    b: Vec<T>,
) -> Vec<T> {
    let mut res = Vec::with_capacity(a.len() + b.len());

    let mut a = a.into_iter();

    let mut b = b.into_iter();

    loop {
        let l = a.as_ref();

        let r = b.as_ref();

        if l.is_empty() || r.is_empty() {
            break;
        }

        if l[0] <= r[0] {
            res.push(a.next().unwrap());
        } else {
            res.push(b.next().unwrap());
        }
    }

    res.extend(a);

    res.extend(b);

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
            assert_eq!(merge_sorted(a, b), ans);
        }
    }
}
