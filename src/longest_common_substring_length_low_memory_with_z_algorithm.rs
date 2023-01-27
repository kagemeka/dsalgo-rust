use crate::z_algorithm::*;

pub fn lcs_len<T: Eq + Clone>(
    a: &[T],
    b: &[T],
) -> usize {
    let n = a.len();

    let mut mx = 0;

    for i in 0..n {
        let mut s = a[i..].to_vec();

        s.extend_from_slice(b);

        mx = mx.max(
            z_algorithm(&s)[n - i..]
                .into_iter()
                .map(|&x| x.min(n - i))
                .max()
                .unwrap(),
        );
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
