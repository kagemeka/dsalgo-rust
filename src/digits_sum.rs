pub fn digits_sum(mut n: u64) -> u64 {
    let mut s = 0;

    while n > 0 {
        s += n % 10;

        n /= 10;
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(10, 1), (100, 1), (123456789, 45)];

        for (n, ans) in cases {
            assert_eq!(digits_sum(n), ans);
        }
    }
}
