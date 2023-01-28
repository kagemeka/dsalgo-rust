use crate::{
    modular_cumprod_usize::*,
    modular_factorial_table_usize::*,
    modular_power_for_prime_usize_recurse::*,
};

pub fn inverse_factorial(
    p: usize,
    size: usize,
) -> Vec<usize> {
    assert!(size <= p);

    let mut a: Vec<_> = (1..size + 1).rev().collect();

    a[0] = pow(p, *factorial(p, size).last().unwrap(), -1);

    a = cumprod(p, a);

    a.reverse();

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let m = 1_000_000_007;

        let ifact = inverse_factorial(m, 20);

        assert_eq!(
            ifact,
            [
                1, 1, 500000004, 166666668, 41666667, 808333339, 301388891,
                900198419, 487524805, 831947206, 283194722, 571199524,
                380933296, 490841026, 320774361, 821384963, 738836565,
                514049213, 639669405, 402087866,
            ]
        );
    }
}
