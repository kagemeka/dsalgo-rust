use crate::{
    longest_common_prefix_array_kasai::lcp_array,
    suffix_array_induced_sort_recurse::sa_is,
};

pub fn sum_of_lcp(a: &[usize]) -> Vec<usize> {
    let n = a.len();

    let mut sa = sa_is(a.to_vec());

    let mut lcp = lcp_array(&a, &sa);

    let mut similarity: Vec<_> = (1..n + 1).rev().collect();

    for _ in 0..2 {
        let mut st = vec![];

        let mut s = 0;

        for i in 0..n - 1 {
            let height = lcp[i];

            let mut len = 1;

            while let Some((h, l)) = st.last() {
                if h < &height {
                    break;
                }

                len += l;

                s -= h * l;

                st.pop();
            }

            s += height * len;

            similarity[sa[i + 1]] += s;

            st.push((height, len));
        }

        sa.reverse();

        lcp.reverse();
    }

    similarity
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/abc213/tasks/abc213_f
        let cases = vec![
            ("abb", vec![3, 3, 2]),
            ("mississippi", vec![11, 16, 14, 12, 13, 11, 9, 7, 4, 3, 4]),
        ];

        for (s, ans) in cases {
            let a: Vec<_> = s.chars().map(|x| x as usize).collect();

            assert_eq!(sum_of_lcp(&a), ans);
        }
    }
}
