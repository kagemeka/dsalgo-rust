pub fn kmp_table<T: PartialEq>(a: &[T]) -> Vec<isize> {
    let n = a.len();

    let mut f = vec![0; n + 1];

    let mut d = -1;

    f[0] = d;

    for i in 0..n {
        while d != -1 && a[d as usize] != a[i] {
            d = f[d as usize]
        }

        d += 1;

        f[i + 1] = if i + 1 < n && a[d as usize] == a[i + 1] {
            f[d as usize]
        } else {
            d
        };
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
            ("abcdabd", vec![-1, 0, 0, 0, -1, 0, 2, 0]),
            ("abacababc", vec![-1, 0, -1, 1, -1, 0, -1, 3, 2, 0]),
            ("abacababa", vec![-1, 0, -1, 1, -1, 0, -1, 3, -1, 3]),
            (
                "participate in parachute",
                vec![
                    -1, 0, 0, 0, 0, 0, 0, -1, 0, 2, 0, 0, 0, 0, 0, -1, 0, 0, 3,
                    0, 0, 0, 0, 0, 0,
                ],
            ),
        ];

        for (s, ans) in cases {
            assert_eq!(kmp_table(s.as_bytes()), ans);
        }
    }
}
