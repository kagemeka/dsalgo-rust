/// O(N\log^2{N})

pub fn suffix_array<T: Ord>(a: &[T]) -> Vec<usize> {
    let mut v: Vec<_> = a.iter().collect();

    v.sort_unstable();

    v.dedup();

    let mut a: Vec<_> =
        a.iter().map(|x| v.binary_search(&x).unwrap() + 1).collect();

    let n = a.len();

    let mut d = 1;

    let mut b: Vec<_> = (0..n as u32).map(|i| (0, i)).collect();

    loop {
        for x in b.iter_mut() {
            x.0 = a[x.1 as usize] << 30
                | a.get(x.1 as usize + d).map_or(0, |r| *r);
        }

        b.sort_unstable_by_key(|&x| x.0);

        d <<= 1;

        if d >= n {
            break;
        }

        let mut rank = 0;

        let mut prev = 0;

        for x in b.iter() {
            if x.0 > prev {
                rank += 1;

                prev = x.0;
            }

            a[x.1 as usize] = rank;
        }

        if rank == n {
            break;
        }
    }

    b.into_iter().map(|x| x.1 as usize).collect()
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
            assert_eq!(suffix_array(&s), ans);
        }
    }
}
