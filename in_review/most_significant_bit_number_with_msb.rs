use crate::most_significant_bit_with_bit_length_u64::msb;

pub fn msb_number(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        1 << msb(n)
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
