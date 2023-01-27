/// \sum_{i=1}^{n} [n/i]

pub fn floor_sum(n: usize) -> usize {
    let mut s = 0;

    let mut x = 1;

    while x * x < n {
        s += x * (n / x - n / (x + 1));

        x += 1;
    }

    for i in 1..=n / x {
        s += n / i;
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
