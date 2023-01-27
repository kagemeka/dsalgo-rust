pub fn z_algorithm_findall<T: PartialEq + Clone>(
    a: &[T],
    pattern: &[T],
) -> Vec<usize> {
    use crate::z_algorithm::z_algorithm;

    let mut s = pattern.to_vec();

    let n = s.len();

    s.append(&mut a.to_vec());

    z_algorithm(&s)[n..]
        .iter()
        .enumerate()
        .filter_map(|(i, &z)| if z >= n { Some(i) } else { None })
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(("ababababc", "aba"), vec![0, 2, 4])];

        for ((s, t), ans) in cases {
            assert_eq!(z_algorithm_findall(s.as_bytes(), t.as_bytes()), ans);
        }
    }
}
