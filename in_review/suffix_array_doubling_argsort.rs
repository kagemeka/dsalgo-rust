use crate::array_compression_unique_binary_search::ArrayCompression;

/// O(N\log^2{N})

pub fn suffix_array(mut a: Vec<usize>) -> Vec<usize> {
    let n = a.len();

    let mut d = 1usize;

    loop {
        for i in 0..n {
            a[i] <<= 30;

            if i + d < n {
                a[i] |= 1 + a[i + d];
            }
        }

        a = ArrayCompression::once(&a);

        d <<= 1;

        if d >= n {
            break;
        }
    }

    let mut sa = (0..n).collect::<Vec<_>>();

    sa.sort_by_key(|&i| a[i]);

    sa
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (
                vec![1, 1, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 2, 2, 0, 0],
                vec![15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
            ),
            (
                vec![1, 0, 3, 3, 0, 3, 3, 0, 2, 2, 0],
                vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2],
            ),
        ];

        for (s, ans) in cases {
            assert_eq!(suffix_array(s), ans);
        }
    }
}
