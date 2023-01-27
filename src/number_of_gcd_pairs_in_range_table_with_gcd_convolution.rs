use crate::gcd_convolution_ring_with_std_multiples_fast_zeta_mobius::*;

pub fn number_of_gcd_pairs(
    lo: usize,
    hi: usize,
) -> Vec<usize> {
    assert!(lo >= 1 && lo <= hi);

    let f: Vec<_> = (0..hi).map(|g| if g < lo { 0 } else { 1 }).collect();

    gcd_convolve(f.clone(), f)
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
    }
}
