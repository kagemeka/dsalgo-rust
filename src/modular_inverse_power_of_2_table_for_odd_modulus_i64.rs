use crate::modular_power_of_k_table::power_of_k;

pub fn power_of_inv_2(
    m: i64,
    size: usize,
) -> Vec<i64> {
    power_of_k(m, (m + 1) >> 1, size)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        dbg!(power_of_inv_2(1_000_000_007, 10));
    }
}
