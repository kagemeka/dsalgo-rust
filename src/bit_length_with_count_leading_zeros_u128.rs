pub fn bit_length(n: u128) -> u8 {
    (0u128.leading_zeros() - n.leading_zeros()) as u8
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
