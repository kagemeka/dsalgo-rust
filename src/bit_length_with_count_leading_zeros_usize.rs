/// O(1)

pub fn bit_length(n: usize) -> usize {
    (0usize.leading_zeros() - n.leading_zeros()) as usize
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
