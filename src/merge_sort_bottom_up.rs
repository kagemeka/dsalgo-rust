pub fn merge_sort<T: Ord + Clone>(mut a: Vec<T>) -> Vec<T> {
    fn merge<T: Ord + Clone>(
        a: &mut [T],
        m: usize,
    ) {
        let n = a.len();

        if n <= m || a[m - 1] <= a[m] {
            return;
        }

        let b = a.to_vec();

        let mut i = 0;

        let mut j = m;

        let mut k = 0;

        while i < m && j < n {
            if b[i] <= b[j] {
                a[k] = b[i].clone();

                i += 1;
            } else {
                a[k] = b[j].clone();

                j += 1;
            }

            k += 1;
        }

        if i < m {
            a[k..n].clone_from_slice(&b[i..m]);
        }
        // even if right is still remain, already sorted.
    }

    let mut d = 1;

    let n = a.len();

    while d < n {
        for i in (0..n).step_by(d << 1) {
            merge(&mut a[i..n.min(i + (d << 1))], d);
        }

        d <<= 1;
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            vec![8, 5, 9, 2, 6, 3, 7, 1, 10, 4],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        )];

        for (a, ans) in cases {
            assert_eq!(merge_sort(a), ans);
        }
    }
}
