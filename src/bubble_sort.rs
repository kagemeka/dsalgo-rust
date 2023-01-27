pub fn bubble_sort<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    let n = a.len();

    for i in 0..n {
        for j in (i..n - 1).rev() {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![4, -1, 2, -3, 0];

        assert_eq!(bubble_sort(a), [-3, -1, 0, 2, 4]);
    }
}
