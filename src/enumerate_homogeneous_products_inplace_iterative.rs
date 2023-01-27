pub fn homogeneous_products(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    assert!(n > 0);

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
            a[j] = a[j - 1];
        }
    }
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
