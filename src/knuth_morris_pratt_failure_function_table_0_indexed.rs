pub fn failure_function<T: PartialEq>(a: &[T]) -> Vec<usize> {
    let n = a.len();

    let mut f = vec![0; n]; // failure function
    let mut d = 0;

    for i in 1..n {
        while d != 0 && a[d] != a[i] {
            d = f[d - 1]
        }

        if a[d] == a[i] {
            d += 1;
        }

        f[i] =
            if d != 0 && i + 1 < n && a[d] == a[i + 1] { f[d - 1] } else { d };
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // refs: en-wiki
        let cases = [
            ("abcdabd", vec![0, 0, 0, 0, 0, 2, 0]),
            ("abacababc", vec![0, 0, 1, 0, 0, 0, 3, 2, 0]),
            ("abacababa", vec![0, 0, 1, 0, 0, 0, 3, 0, 3]),
            (
                "participate in parachute",
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                    0, 0, 0, 0,
                ],
            ),
        ];

        for (s, ans) in cases {
            assert_eq!(failure_function(s.as_bytes()), ans);
        }
    }
}
