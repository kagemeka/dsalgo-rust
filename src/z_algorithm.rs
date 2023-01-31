pub fn z_algorithm<T: PartialEq>(a: &[T]) -> Vec<usize> {
    let n = a.len();

    let mut lcp = vec![0; n];

    let mut l = 0;

    for i in 1..n {
        let r = l + lcp[l];

        let mut d = if r <= i { 0 } else { lcp[i - l].min(r - i) };

        while i + d < n && a[i + d] == a[d] {
            d += 1;
        }

        lcp[i] = d;

        if i + d > r {
            l = i;
        }
    }

    lcp[0] = n;

    lcp
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [
            ("aabcaabxaaaz", vec![12, 1, 0, 0, 3, 1, 0, 0, 2, 2, 1, 0]),
            ("aaaaaa", vec![6, 5, 4, 3, 2, 1]),
            ("aabaacd", vec![7, 1, 0, 2, 1, 0, 0]),
            ("abababab", vec![8, 0, 6, 0, 4, 0, 2, 0]),
        ];

        for (s, ans) in cases {
            let s = s.chars().collect::<Vec<_>>();

            assert_eq!(z_algorithm(&s), ans);
        }
    }
}
