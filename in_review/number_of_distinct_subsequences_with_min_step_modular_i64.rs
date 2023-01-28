pub fn number_of_subsequences<T: std::hash::Hash + Eq>(
    modulus: i64,
    a: &[T],
    min_step: usize,
) -> i64 {
    let n: usize = a.len();

    let mut dp = vec![0; n + 1];

    dp[0] = 1;

    let mut prev = std::collections::HashMap::new();

    for i in 0..n {
        dp[i + 1] = dp[i] + dp[i.max(min_step) - min_step];

        let j = prev.entry(&a[i]).or_insert(n);

        if *j != n {
            dp[i + 1] -= dp[(*j).max(min_step) - min_step];
        }

        dp[i + 1] %= modulus;

        *j = i;
    }

    (*dp.last().unwrap() + modulus) % modulus
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        let cases = vec![
            (("abc", 1), 5),
            (("aa", 1), 2),
            (("acba", 1), 7),
            (("chokudai", 1), 55),
        ];

        for ((s, k), ans) in cases {
            assert_eq!(number_of_subsequences(MOD, s.as_bytes(), k), ans);
        }
    }
}
