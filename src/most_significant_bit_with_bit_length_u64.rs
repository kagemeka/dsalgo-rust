use crate::bit_length_with_count_leading_zeros_u64::bit_length;

/// O(1)

pub fn msb(n: u64) -> usize {
    assert!(n > 0);

    bit_length(n) as usize - 1
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
