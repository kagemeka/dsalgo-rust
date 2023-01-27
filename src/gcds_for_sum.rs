use crate::find_divisors_trial_division_u64::find_divisors;

/// a_i > 0, lcm_prod(a_i) = sum.

pub fn gcds_for_sum(
    n: usize,
    sum: u64,
) -> Vec<u64> {
    find_divisors(sum).into_iter().filter(|&x| x * n as u64 <= sum).collect()
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
