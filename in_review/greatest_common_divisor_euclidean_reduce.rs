use std::ops::*;

use crate::greatest_common_divisor_euclidean::gcd;

pub fn gcd_reduce<T, I>(iter: I) -> T
where
    I: Iterator<Item = T>,
    T: Default + PartialEq + Copy + RemAssign + PartialOrd,
{
    iter.fold(T::default(), |a, b| gcd(a, b))
}

#[cfg(test)]

mod tests {

    use super::*;

    const CASES: &[(&[i32], i32)] = &[
        (&[], 0),
        (&[0], 0),
        (&[10], 10),
        (&[0, 2, 8, 4], 2),
        (&[33, -111], 3),
        (&[-33, 111], 3),
        (&[-33, -111], 3),
        (&[100, -3], 1),
        (&[-1, 0], 1),
        (&[10, 5], 5),
        (&[0, 10], 10),
    ];

    #[test]

    fn test_reduce() {
        for &(values, ans) in CASES {
            assert_eq!(gcd_reduce(values.iter().cloned()).abs(), ans);
        }
    }
}
