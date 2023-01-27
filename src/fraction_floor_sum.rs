/// \sum_{i=1}^{n} [n/i]

pub fn floor_sum(n: usize) -> usize {
    let mut i = 1;

    let mut s = 0;

    while i <= n {
        let x = n / i;

        let ni = n / x + 1;

        s += x * (ni - i);

        i = ni;
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(3, 5), (10000000000, 231802823220)];

        for (n, ans) in cases {
            assert_eq!(floor_sum(n), ans);
        }
    }
}
