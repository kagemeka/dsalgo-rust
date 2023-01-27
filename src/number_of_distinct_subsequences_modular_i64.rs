pub fn number_of_subsequences<T: std::hash::Hash + Eq>(
    m: i64,
    a: &[T],
) -> i64 {
    let n: usize = a.len();

    let mut dp = vec![0; n + 1];

    dp[0] = 1;

    let mut prev = std::collections::HashMap::new();

    for i in 0..n {
        dp[i + 1] = dp[i] * 2;

        let j = prev.entry(&a[i]).or_insert(n);

        if *j != n {
            dp[i + 1] -= dp[*j];
        }

        dp[i + 1] %= m;

        *j = i;
    }

    (*dp.last().unwrap() + m) % m
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let m = 1_000_000_007;

        let cases = vec![(vec![1, 2, 3, 2, 4], 28)];

        for (a, ans) in cases {
            assert_eq!(number_of_subsequences(m, &a), ans);
        }
    }
}
