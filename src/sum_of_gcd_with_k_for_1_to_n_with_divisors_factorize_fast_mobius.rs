use crate::{
    find_divisors_trial_division_u64::find_divisors,
    prime_factorize_trial_division::prime_factorize,
};

/// sum(gcd(i, k)) for i = 1..=n

pub fn sum_of_gcd(
    n: u64,
    k: u64,
) -> u64 {
    let divs = find_divisors(k);

    let mut cnt = std::collections::HashMap::new();

    for &d in divs.iter() {
        cnt.insert(d, n / d);
    }

    for (p, _) in prime_factorize(k) {
        for &d in divs.iter() {
            if d % p == 0 {
                let v = *cnt.get(&d).unwrap();

                *cnt.get_mut(&(d / p)).unwrap() -= v;
            }
        }
    }

    let mut s = 0;

    for d in divs.iter() {
        s += cnt.get(d).unwrap() * d;
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
