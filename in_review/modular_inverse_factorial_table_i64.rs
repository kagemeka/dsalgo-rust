use crate::{
    modular_cumprod_i64::*,
    modular_factorial_table_i64::*,
    modular_power_with_neg_exp_i32::*,
};

/// modular m must be coprime with n < m;
/// so lpf(m) >= size

pub fn inverse_factorial(
    m: i64,
    size: usize,
) -> Vec<i64> {
    assert!(size <= m as usize);

    let mut a: Vec<_> = (1..size as i64 + 1).rev().collect();

    a[0] = pow(m as i32, *factorial(m, size).last().unwrap(), -1) as i64;

    a = cumprod(m, a);

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
