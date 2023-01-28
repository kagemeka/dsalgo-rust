pub fn number_of_divisors(
    modulus: usize,
    factors: &[(usize, usize)],
) -> usize {
    let mut c = 1;

    for (_, e) in factors.iter() {
        c *= e + 1;

        c %= modulus;
    }

    c
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let m = 1_000_000_007;

        let cases =
            vec![(vec![(2, 2), (3, 1)], 6), (vec![(2, 1), (3, 1), (5, 1)], 8)];

        for (factors, ans) in cases {
            assert_eq!(number_of_divisors(m, &factors), ans);
        }
    }
}
