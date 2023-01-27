pub fn number_of_swap<T: Ord>(mut a: Vec<T>) -> usize {
    let n = a.len();

    let mut cnt = 0;

    for i in 0..n - 1 {
        let mut k = i;

        for j in i + 1..n {
            if a[j] < a[k] {
                k = j;
            }
        }

        if k > i {
            cnt += 1;

            a.swap(i, k);
        }
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(vec![5, 2, 4, 6, 1, 3], 3), (vec![1, 2, 3], 0)];

        for (a, ans) in cases {
            assert_eq!(number_of_swap(a), ans);
        }
    }
}
