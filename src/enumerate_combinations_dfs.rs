pub fn combinations(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    assert!(k <= n);

    let mut res = vec![];

    let mut st = vec![vec![]];

    while let Some(a) = st.pop() {
        let m = a.len();

        if m == k {
            res.push(a);

            continue;
        }

        let lo = if m == 0 { 0 } else { a[m - 1] + 1 };

        for x in (lo..n).rev() {
            let mut b = a.clone();

            b.push(x);

            st.push(b);
        }
    }

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let res = combinations(5, 3);

        assert_eq!(
            res,
            [
                [0, 1, 2],
                [0, 1, 3],
                [0, 1, 4],
                [0, 2, 3],
                [0, 2, 4],
                [0, 3, 4],
                [1, 2, 3],
                [1, 2, 4],
                [1, 3, 4],
                [2, 3, 4]
            ]
        );
    }
}
