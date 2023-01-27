pub fn kmp_findall<T: PartialEq>(
    a: &[T],
    pattern: &[T],
) -> Vec<usize> {
    use crate::knuth_morris_pratt_failure_function_table_0_indexed::*;

    let b = pattern;

    let (n, m) = (a.len(), b.len());

    let f = failure_function(b);

    let mut indices = vec![];

    let mut j = 0;

    for i in 0..n {
        while j != 0 && b[j] != a[i] {
            j = f[j - 1];
        }

        if b[j] == a[i] {
            j += 1;
        }

        if j == m {
            indices.push(i + 1 - m);

            j = f[m - 1];
        }
    }

    indices
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(("ababababc", "aba"), vec![0, 2, 4])];

        for ((s, t), ans) in cases {
            assert_eq!(kmp_findall(s.as_bytes(), t.as_bytes()), ans);
        }
    }
}
