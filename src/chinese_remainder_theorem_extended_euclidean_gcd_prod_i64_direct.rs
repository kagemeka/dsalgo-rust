use crate::extended_euclidean_gcd_i64_recurse::extgcd;

pub fn crt(mr: &[(i64, i64)]) -> Option<(i64, i64)> {
    use std::mem::swap;

    let (mut m, mut r) = (1, 0);

    for &(mut m1, mut r1) in mr.iter() {
        assert!(m1 > 0);

        r1 %= m1;

        if m < m1 {
            // needed to avoid overflow
            swap(&mut r, &mut r1);

            swap(&mut m, &mut m1);
        }

        if m % m1 == 0 {
            if r % m1 == r1 {
                continue;
            }

            return None;
        }

        let (g, inv_u, _) = extgcd(m, m1);

        if (r1 - r) % g != 0 {
            return None;
        }

        let u1 = m1 / g;

        let x = (r1 - r) / g * inv_u % u1;

        r += x * m;

        m *= u1;
    }

    if r < 0 {
        r += m;
    }

    debug_assert!(0 <= r && r < m);

    Some((m, r))
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
            assert_eq!(crt(&mr), ans);
        }
    }
}
