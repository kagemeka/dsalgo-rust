pub fn lsb(n: u64) -> usize {
    assert!(n > 0);

    let n = n as isize;

    (n & -n).trailing_zeros() as usize
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(1, 0), (0b01010, 1)];

        for (n, ans) in cases {
            assert_eq!(lsb(n), ans);
        }
    }
}
