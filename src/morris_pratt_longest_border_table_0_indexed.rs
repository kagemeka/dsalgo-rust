pub fn longest_border<T: PartialEq>(a: &[T]) -> Vec<usize> {
    let n = a.len();

    let mut lb = vec![0; n];

    let mut d = 0;

    for i in 1..n {
        while d != 0 && a[d] != a[i] {
            d = lb[d - 1];
        }

        if a[d] == a[i] {
            d += 1;
        }

        lb[i] = d;
    }

    lb
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // refs: en-wiki
        let cases = [
            ("aaaaa", vec![0, 1, 2, 3, 4]),
            ("abcdabd", vec![0, 0, 0, 0, 1, 2, 0]),
            ("abacababc", vec![0, 0, 1, 0, 1, 2, 3, 2, 0]),
            ("abacababa", vec![0, 0, 1, 0, 1, 2, 3, 2, 3]),
            (
                "participate in parachute",
                vec![
                    0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 1, 2, 3, 0, 0,
                    0, 0, 0, 0,
                ],
            ),
        ];

        for (s, ans) in cases {
            assert_eq!(longest_border(s.as_bytes()), ans);
        }
    }
}
