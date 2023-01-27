use crate::least_common_multiple_with_gcd_u64::lcm;

pub fn lcm_reduce<I>(iter: I) -> u64
where
    I: Iterator<Item = u64>,
{
    iter.fold(1, |a, b| lcm(a, b))
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
