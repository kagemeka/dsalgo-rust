use crate::fast_zeta_transform_for_subset_additive_with_std::fast_zeta_subset;

pub fn popcount(size: usize) -> Vec<u8> {
    let mut f = vec![0; size];

    let n = size.next_power_of_two().trailing_zeros();

    for i in 0..n {
        f[1 << i] = 1;
    }

    fast_zeta_subset(f)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::popcount_table::popcount as naive;

        for i in 1..100 {
            assert_eq!(popcount(i), naive(i));
        }
    }
}
