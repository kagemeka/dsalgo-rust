pub fn mul(
    a: &[Vec<i64>],
    b: &[Vec<i64>],
) -> Vec<Vec<i64>> {
    let h = a.len();

    let n = a[0].len();

    assert_eq!(b.len(), n);

    let w = b[0].len();

    let mut c = vec![vec![0; w]; h];

    for i in 0..h {
        for k in 0..n {
            for j in 0..w {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![vec![0, 1], vec![2, 3], vec![4, 5]];

        let b = vec![vec![0, 1, 2], vec![3, 4, 5]];

        let c = mul(&a, &b);

        assert_eq!(c, [[3, 4, 5,], [9, 14, 19,], [15, 24, 33,]]);
    }
}
