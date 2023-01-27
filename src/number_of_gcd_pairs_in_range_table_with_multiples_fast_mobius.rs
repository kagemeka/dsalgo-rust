use crate::fast_mobius_transform_for_multiples_additive_with_std::*;

/// \forall 1 <= g < hi, f(g) = \sum_{lo <= a, b < hi, gcd(a, b) = g}{1}
/// \zeta{f(g)} := \sum_{g|x}\sum_{lo <= a, b < hi, gcd(a, b) = x}{1} (easy)
/// f(g) = mobius_transform(\zeta{f(g)})

pub fn number_of_gcd_pairs(
    mut lo: usize,
    mut hi: usize,
) -> Vec<usize> {
    assert!(lo >= 1 && lo <= hi);

    lo -= 1;

    hi -= 1;

    let f: Vec<_> = (0..=hi)
        .map(|g| if g == 0 { 0 } else { (hi / g - lo / g).pow(2) })
        .collect();

    fast_mobius_multiples(f)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            number_of_gcd_pairs(1, 10),
            [0, 55, 11, 7, 3, 1, 1, 1, 1, 1]
        );
        // example
        // g = 2,
        // (a, b) = (2, (2, 4, 6, 8)), (4, (2, 6)), (6, (2, 4, 8)), (8, (2, 6))
    }
}
