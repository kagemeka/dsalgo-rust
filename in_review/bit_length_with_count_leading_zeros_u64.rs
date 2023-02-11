/// O(1)

pub fn bit_length(n: u64) -> u8 {
    (0u64.leading_zeros() - n.leading_zeros()) as u8
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
