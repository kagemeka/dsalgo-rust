pub fn combinations(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    assert!(k <= n);

    let mut res = vec![];

    fn dfs(
        n: usize,
        k: usize,
        res: &mut Vec<Vec<usize>>,
        a: &mut Vec<usize>,
    ) {
        let m = a.len();

        if m == k {
            res.push(a.to_vec());

            return;
        }

        let lo = if m == 0 { 0 } else { a[m - 1] + 1 };

        for x in lo..n {
            a.push(x);

            dfs(n, k, res, a);

            a.pop();
        }
    }

    dfs(n, k, &mut res, &mut vec![]);

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
