use crate::legendre_formula_u64::legendre;

/// trailing zeros in decimal.

pub fn count_factorial_trailing_zeros(n: u64) -> u64 { legendre(n, 5) }

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
