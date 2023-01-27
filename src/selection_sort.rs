pub fn selection_sort<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    let n = a.len();

    for i in 0..n - 1 {
        let mut k = i;

        for j in i + 1..n {
            if a[j] < a[k] {
                k = j;
            }
        }

        a.swap(i, k);
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (vec![5, 2, 4, 6, 1, 3], vec![1, 2, 3, 4, 5, 6]),
            (vec![1, 2, 3], vec![1, 2, 3]),
        ];

        for (a, ans) in cases {
            assert_eq!(selection_sort(a), ans);
        }
    }
}
