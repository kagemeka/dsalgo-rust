use crate::lcm_convolution_ring_with_std_divisors_fast_zeta_mobius::*;

pub fn number_of_lcm_pairs(
    lo: usize,
    hi: usize,
) -> Vec<usize> {
    assert!(lo >= 1 && lo <= hi);

    let f: Vec<_> = (0..hi).map(|g| if g < lo { 0 } else { 1 }).collect();

    lcm_convolve(f.clone(), f)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(number_of_lcm_pairs(1, 10), [0, 1, 3, 3, 5, 3, 9, 3, 7, 5]);
    }
}
