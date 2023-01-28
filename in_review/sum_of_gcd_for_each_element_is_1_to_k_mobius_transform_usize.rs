use crate::power_multiplicative_semigroup_with_std_ops::power;

/// n is length of array.

pub fn sum_of_gcd(
    k: usize,
    n: u64,
) -> usize {
    let mut d = vec![0; k + 1];

    let mut s = 0;

    for i in (1..=k).rev() {
        d[i] = power(k / i, n);

        for j in (i << 1..=k).step_by(i) {
            d[i] -= d[j];
        }

        s += d[i] * i;
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [(2, 3, 9), (200, 3, 10813692)];

        for (k, n, ans) in cases {
            assert_eq!(sum_of_gcd(k, n), ans);
        }
    }
}
