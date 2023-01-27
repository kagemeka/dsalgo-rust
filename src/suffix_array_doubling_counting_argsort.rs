pub fn suffix_array<T: Ord>(a: &[T]) -> Vec<usize> {
    let n = a.len();

    fn counting_argsort(a: &Vec<usize>) -> Vec<usize> {
        let n = a.len();

        let mut arg = vec![0; n + 2];

        for &x in a.iter() {
            arg[x + 1] += 1;
        }

        let mut key = vec![0; n];

        for i in 0..n {
            arg[i + 1] += arg[i];
        }

        for (i, &x) in a.iter().enumerate() {
            key[arg[x]] = i;

            arg[x] += 1;
        }

        key
    }

    let mut v = Vec::with_capacity(n);

    for x in a.iter() {
        v.push(x);
    }

    v.sort_unstable();

    v.dedup();

    let mut a: Vec<_> =
        a.iter().map(|x| v.binary_search(&x).unwrap() + 1).collect();

    let mut k = 1usize;

    loop {
        let key_1 =
            (0..n).map(|i| if i + k < n { a[i + k] } else { 0 }).collect();

        let sa_1 = counting_argsort(&key_1);

        let key_0 = sa_1.iter().map(|&i| a[i]).collect();

        let sa_0 = counting_argsort(&key_0);

        let sa: Vec<_> = sa_0.iter().map(|&i| sa_1[i]).collect();

        let key: Vec<_> = sa_0
            .iter()
            .zip(sa.iter())
            .map(|(&i, &j)| key_0[i] << 30 | key_1[j])
            .collect();

        k <<= 1;

        if k >= n {
            return sa;
        }

        let mut rank = 0;

        let mut prev = 0;

        for (&sa, &key) in sa.iter().zip(key.iter()) {
            if key > prev {
                rank += 1;

                prev = key;
            }

            a[sa] = rank;
        }

        if rank == n {
            return sa;
        }
    }
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
