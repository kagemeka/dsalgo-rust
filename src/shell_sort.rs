use crate::shell_sort_gap_sequences::CIURA_2001 as gaps;

pub fn shell_sort<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    let n = a.len();

    for &g in gaps.iter().rev() {
        for i in g..n {
            let mut j = i;

            while j >= g {
                if a[j - g] <= a[j] {
                    break;
                }

                a.swap(j - g, j);

                j -= g;
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
        let cases = vec![
            (vec![5, 2, 4, 6, 1, 3], vec![1, 2, 3, 4, 5, 6]),
            (vec![1, 2, 3], vec![1, 2, 3]),
        ];

        for (a, ans) in cases {
            assert_eq!(shell_sort(a), ans);
        }
    }
}
