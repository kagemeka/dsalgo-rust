use crate::{
    modular_factorial_table_usize::factorial,
    modular_inverse_factorial_table_usize::inverse_factorial,
};

/// inv[0] is undefined. please don't access to

pub fn inverse(
    m: usize,
    size: usize,
) -> Vec<usize> {
    let mut inv = inverse_factorial(m, size);

    for (i, x) in factorial(m, size - 1).into_iter().enumerate() {
        inv[i + 1] *= x;

        inv[i + 1] %= m;
    }

    inv
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: usize = 1_000_000_007;

        let inv = inverse(MOD, 20);

        assert_eq!(
            inv,
            [
                1, 1, 500000004, 333333336, 250000002, 400000003, 166666668,
                142857144, 125000001, 111111112, 700000005, 818181824,
                83333334, 153846155, 71428572, 466666670, 562500004, 352941179,
                55555556, 157894738,
            ]
        );
    }
}
