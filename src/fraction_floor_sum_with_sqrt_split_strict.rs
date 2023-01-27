use crate::integer_square_root_with_binary_search_usize::*;

/// \sum_{i=1}^{n} [n/i]

pub fn floor_sum(n: usize) -> usize {
    let mut s = 0;

    let k = isqrt(n);

    for i in 1..n / (k + 1) + 1 {
        s += n / i;
    }

    for x in 1..k + 1 {
        s += x * (n / x - n / (x + 1))
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
