pub fn sum_of_lcp<T: PartialEq + Clone>(
    a: &[T],
    pattern: &[T],
) -> usize {
    use crate::z_algorithm::z_algorithm;

    let mut s = pattern.to_vec();

    let n = s.len();

    s.append(&mut a.to_vec());

    z_algorithm(&s)[n..].iter().map(|x| x.min(&n)).sum()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(("ababababc", "aba"), 11)];

        for ((s, t), ans) in cases {
            assert_eq!(sum_of_lcp(s.as_bytes(), t.as_bytes()), ans);
        }
    }
}
