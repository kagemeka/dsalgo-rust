pub fn lsb_num(n: usize) -> usize {
    match n {
        0 => 0,
        n => 1 << n.trailing_zeros(),
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(0, 0), (1, 1), (2, 2), (3, 1)];

        for (n, ans) in cases {
            assert_eq!(lsb_num(n), ans);
        }
    }
}
