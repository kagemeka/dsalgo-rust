pub fn next_power_of_2(n: u64) -> u64 { n.next_power_of_two() }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::next_power_of_2_table_const_usize::NEXT_POWER_OF_2;

        for (i, &ans) in NEXT_POWER_OF_2.iter().enumerate() {
            assert_eq!(next_power_of_2(i as u64) as usize, ans);
        }
    }
}
