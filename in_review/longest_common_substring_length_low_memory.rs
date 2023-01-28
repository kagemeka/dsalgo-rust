pub fn lcs_len<T: Eq>(
    a: &[T],
    b: &[T],
) -> usize {
    let n = a.len();

    let m = b.len();

    if n < m {
        return lcs_len(b, a);
    }

    let mut dp = vec![0; m + 1];

    let mut mx = 0;

    for i in 0..n {
        for j in (0..m).rev() {
            dp[j + 1] = if a[i] == b[j] { dp[j] + 1 } else { 0 }
        }

        mx = mx.max(*dp.iter().max().unwrap());
    }

    mx
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(("abcaba", "acaca"), 2)];

        for ((s, t), ans) in cases {
            assert_eq!(lcs_len(s.as_bytes(), t.as_bytes()), ans);
        }
    }
}
