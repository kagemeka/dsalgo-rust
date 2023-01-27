use crate::shell_sort_gap_sequences::CIURA_2001 as gaps;

pub fn number_of_swap<T: Ord>(mut a: Vec<T>) -> usize {
    let n = a.len();

    let mut cnt = 0;

    for &g in gaps.iter().rev() {
        for i in g..n {
            let mut j = i;

            while j >= g {
                if a[j - g] <= a[j] {
                    break;
                }

                a.swap(j - g, j);

                cnt += 1;

                j -= g;
            }
        }
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(vec![5, 2, 4, 6, 1, 3], 5), (vec![1, 2, 3], 0)];

        for (a, ans) in cases {
            assert_eq!(number_of_swap(a), ans);
        }
    }
}
