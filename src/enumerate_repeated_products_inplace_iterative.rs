pub fn repeated_products(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    if n == 0 {
        assert!(k == 0);
    }

    let mut res = vec![];

    let mut a: Vec<_> = vec![0; k];

    loop {
        res.push(a.clone());

        let mut i = k;

        for j in (0..k).rev() {
            if a[j] != n - 1 {
                i = j;

                break;
            }
        }

        if i == k {
            return res;
        }

        a[i] += 1;

        for j in i + 1..k {
            a[j] = 0;
        }
    }
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
