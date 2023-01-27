pub fn homogeneous_products(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    assert!(n > 0);

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

        let lo = if m == 0 { 0 } else { a[m - 1] };

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
        let res = homogeneous_products(5, 3);

        assert_eq!(
            res,
            [
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 2],
                [0, 0, 3],
                [0, 0, 4],
                [0, 1, 1],
                [0, 1, 2],
                [0, 1, 3],
                [0, 1, 4],
                [0, 2, 2],
                [0, 2, 3],
                [0, 2, 4],
                [0, 3, 3],
                [0, 3, 4],
                [0, 4, 4],
                [1, 1, 1],
                [1, 1, 2],
                [1, 1, 3],
                [1, 1, 4],
                [1, 2, 2],
                [1, 2, 3],
                [1, 2, 4],
                [1, 3, 3],
                [1, 3, 4],
                [1, 4, 4],
                [2, 2, 2],
                [2, 2, 3],
                [2, 2, 4],
                [2, 3, 3],
                [2, 3, 4],
                [2, 4, 4],
                [3, 3, 3],
                [3, 3, 4],
                [3, 4, 4],
                [4, 4, 4]
            ]
        );
    }
}
