use crate::{
    longest_common_prefix_array_kasai::lcp_array,
    suffix_array_induced_sort_recurse::sa_is,
};

pub fn count_substrings(a: &[usize]) -> usize {
    let sa = sa_is(a.to_vec());

    let lcp = lcp_array(&a, &sa);

    let n = sa.len();

    n * (n + 1) / 2 - lcp.iter().sum::<usize>()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/practice2/tasks/practice2_i
        let cases = vec![
            ("abcbcba", 21),
            ("mississippi", 53),
            ("ababacaca", 33),
            ("aaaaa", 5),
        ];

        for (a, ans) in cases {
            let a: Vec<_> = a.chars().map(|x| x as usize).collect();

            assert_eq!(count_substrings(&a), ans);
        }
    }
}
