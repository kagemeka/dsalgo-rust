use crate::{
    find_divisors_trial_division_u64::find_divisors,
    greatest_common_divisor_euclidean_reduce::gcd_reduce,
};

pub fn find_divisors_for_same_remainders<I>(mut iter: I) -> Vec<u64>
where
    I: Iterator<Item = u64>,
{
    if let Some(a0) = iter.next() {
        find_divisors(gcd_reduce(iter.map(|a| {
            if a >= a0 {
                a - a0
            } else {
                a0 - a
            }
        })))
    } else {
        vec![]
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;

        assert_eq!(
            find_divisors_for_same_remainders([100, 30].into_iter()),
            [1, 2, 5, 7, 10, 14, 35, 70],
        );
    }
}
