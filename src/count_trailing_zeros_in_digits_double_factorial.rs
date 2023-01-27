pub fn ctz_digits_double_factorial(n: u64) -> u64 {
    if n & 1 == 1 {
        return 0;
    }

    let mut cnt = 0;

    let mut p = 2;

    while p < n {
        p *= 5;

        cnt += n / p;
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases =
            vec![(12, 1), (5, 0), (1000000000000000000, 124999999999999995)];

        for (n, ans) in cases {
            assert_eq!(ctz_digits_double_factorial(n), ans);
        }
    }
}
