pub fn repeated_products(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    if n == 0 {
        assert!(k == 0);
    }

    let mut res = vec![];

    fn dfs(
        n: usize,
        k: usize,
        res: &mut Vec<Vec<usize>>,
        a: &mut Vec<usize>,
    ) {
        if a.len() == k {
            res.push(a.to_vec());

            return;
        }

        for x in 0..n {
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
        let res = repeated_products(3, 3);

        assert_eq!(
            res,
            [
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 2],
                [0, 1, 0],
                [0, 1, 1],
                [0, 1, 2],
                [0, 2, 0],
                [0, 2, 1],
                [0, 2, 2],
                [1, 0, 0],
                [1, 0, 1],
                [1, 0, 2],
                [1, 1, 0],
                [1, 1, 1],
                [1, 1, 2],
                [1, 2, 0],
                [1, 2, 1],
                [1, 2, 2],
                [2, 0, 0],
                [2, 0, 1],
                [2, 0, 2],
                [2, 1, 0],
                [2, 1, 1],
                [2, 1, 2],
                [2, 2, 0],
                [2, 2, 1],
                [2, 2, 2]
            ]
        );
    }
}
