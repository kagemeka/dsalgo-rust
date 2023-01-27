use crate::chinese_remainder_theorem_extended_euclidean_gcd_safe_usize::*;

pub fn crt_prod(mr: &[(usize, usize)]) -> Option<(usize, usize)> {
    let (mut m0, mut r0) = (1, 0);

    for &(m1, r1) in mr.iter() {
        let (lcm, ans) = safe_crt(m0, r0, m1, r1)?;

        m0 = lcm;

        r0 = ans;

        debug_assert!(r0 < m0);
    }

    Some((m0, r0))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (vec![(5, 3), (7, 4), (8, 3)], Some((280, 123))),
            (vec![], Some((1, 0))),
            (vec![(20, 10), (30, 10), (40, 10)], Some((120, 10))),
            (vec![(20, 10), (30, 10), (40, 30)], Some((120, 70))),
            (vec![(2, 1), (4, 0), (17, 5)], None),
            (
                vec![(221549, 80712), (699312, 320302), (496729, 140367)],
                Some((76959154983203952, 38774484298448350)),
            ),
        ];

        for (mr, ans) in cases {
            assert_eq!(crt_prod(&mr), ans);
        }
    }
}
