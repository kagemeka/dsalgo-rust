pub fn next_combination(s: usize) -> usize {
    assert!(s > 0);

    let s = s as isize;

    let lsb_num = 1 << s.trailing_zeros();

    let t = s + lsb_num;

    ((s & !t) / lsb_num >> 1 | t) as usize
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut s = 0b000111;

        for _ in 0..10 {
            println!("{:06b}, {}", s, s);

            s = next_combination(s);
        }
    }

    #[test]

    fn test_bit_ops() {
        let s = 0b0110100100110isize;

        let lsb_num = s & -s;

        assert_eq!(lsb_num, 0b0000000000010);

        let t = s + lsb_num;

        assert_eq!(t, 0b0110100101000);

        let continuous_set_bits_from_lsb = s & !t;

        assert_eq!(continuous_set_bits_from_lsb, 0b0000000000110);

        let pop_ctz = continuous_set_bits_from_lsb / lsb_num;

        assert_eq!(pop_ctz, 0b000000000011);
    }
}
