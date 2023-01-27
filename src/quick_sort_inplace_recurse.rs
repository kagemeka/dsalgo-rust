pub fn quick_sort<T: Ord>(a: &mut [T]) {
    /// [lo, hi)

    fn partition<T: Ord>(
        a: &mut [T],
        lo: usize,
        hi: usize,
    ) -> usize {
        debug_assert!(lo < hi);

        let mut i = lo;

        for j in lo..hi {
            if a[j] > a[hi - 1] {
                continue;
            }

            a.swap(i, j);

            i += 1;
        }

        i
    }

    fn sort<T: Ord>(
        a: &mut [T],
        lo: usize,
        hi: usize,
    ) {
        if hi - lo <= 1 {
            return;
        }

        let p = partition(a, lo, hi);

        sort(a, lo, p - 1);

        sort(a, p, hi);
    }

    sort(a, 0, a.len());
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

        for (mut a, ans) in cases {
            quick_sort(&mut a);

            assert_eq!(a, ans);
        }
    }
}
