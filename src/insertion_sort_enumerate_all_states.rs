pub fn insertion_sort_states<T: Ord + Clone>(mut a: Vec<T>) -> Vec<Vec<T>> {
    let n = a.len();

    let mut res = vec![a.clone()];

    for i in 1..n {
        for j in (0..i).rev() {
            if a[j] <= a[j + 1] {
                break;
            }

            a.swap(j, j + 1);
        }

        res.push(a.clone());
    }

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (
                vec![5, 2, 4, 6, 1, 3],
                vec![
                    vec![5, 2, 4, 6, 1, 3],
                    vec![2, 5, 4, 6, 1, 3],
                    vec![2, 4, 5, 6, 1, 3],
                    vec![2, 4, 5, 6, 1, 3],
                    vec![1, 2, 4, 5, 6, 3],
                    vec![1, 2, 3, 4, 5, 6],
                ],
            ),
            (vec![1, 2, 3], vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]),
        ];

        for (a, ans) in cases {
            assert_eq!(insertion_sort_states(a), ans);
        }
    }
}
