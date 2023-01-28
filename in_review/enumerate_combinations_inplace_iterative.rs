pub fn combinations(
    n: usize,
    k: usize,
) -> Vec<Vec<usize>> {
    assert!(k <= n);

    let mut res = vec![];

    let mut a: Vec<_> = (0..k).collect();

    loop {
        res.push(a.clone());

        let mut i = k;

        for j in (0..k).rev() {
            if a[j] != j + n - k {
                i = j;

                break;
            }
        }

        if i == k {
            return res;
        }

        a[i] += 1;

        for j in i + 1..k {
            a[j] = a[j - 1] + 1;
        }
    }
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
