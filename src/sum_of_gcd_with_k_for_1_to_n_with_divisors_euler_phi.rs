use crate::{
    euler_totient_with_trial_division_u64::phi,
    find_divisors_trial_division_u64::find_divisors,
};

/// sum(gcd(i, k)) for i = 1..=n

pub fn sum_of_gcd(
    n: u64,
    k: u64,
) -> u64 {
    let mut s = 0;

    for d in find_divisors(k).into_iter() {
        s += n / d * phi(d);
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;
    use crate::greatest_common_divisor_euclidean_u64::gcd;

    #[test]

    fn test() {
        let cases = [(1000, 30)];

        for (n, k) in cases {
            let mut ans = 0;

            for i in 1..=n {
                ans += gcd(i, k);
            }

            assert_eq!(sum_of_gcd(n, k), ans);
        }
    }
}
