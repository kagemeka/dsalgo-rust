/// from 1-indexed dp table

pub fn find_all_ends(dp: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut mx = 0;

    for row in dp.iter() {
        mx = mx.max(*row.iter().max().unwrap());
    }

    let mut ends = vec![];

    if mx == 0 {
        return ends;
    }

    for (i, row) in dp.iter().enumerate() {
        for (j, &lcs) in row.iter().enumerate() {
            if lcs == mx {
                ends.push((i - 1, j - 1));
            }
        }
    }

    ends
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::longest_common_substring_table::*;

        let cases = vec![(("aabbcc", "ccbbaa"), vec![(1, 5), (3, 3), (5, 1)])];

        for ((s, t), ans) in cases {
            assert_eq!(
                find_all_ends(&longest_common_substring(
                    s.as_bytes(),
                    t.as_bytes()
                )),
                ans
            );
        }
    }
}
