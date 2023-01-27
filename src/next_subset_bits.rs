pub fn next_subset_bits(
    s: usize,
    t: usize,
) -> usize {
    assert!(t > 0);

    (t - 1) & s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let s = 0b0110100101;

        let mut t = s;

        while t > 0 {
            println!("{:010b}", t);

            t = next_subset_bits(s, t);
        }
    }
}
