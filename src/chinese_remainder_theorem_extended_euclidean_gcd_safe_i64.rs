use crate::extended_euclidean_gcd_i64_recurse::extgcd;

pub fn safe_crt(
    mut m0: i64,
    mut r0: i64,
    mut m1: i64,
    mut r1: i64,
) -> Option<(i64, i64)> {
    use std::mem::swap;

    assert!(0 < m0 && 0 < m1);

    r0 %= m0;

    r1 %= m1;

    if m0 < m1 {
        // needed to avoid overflow
        swap(&mut r0, &mut r1);

        swap(&mut m0, &mut m1);
    }

    if m0 % m1 == 0 {
        return if r0 % m1 == r1 { Some((m0, r0)) } else { None };
    }

    let (g, inv_u0, _) = extgcd(m0, m1);

    if (r1 - r0) % g != 0 {
        return None;
    }

    let u1 = m1 / g;

    let x = (r1 - r0) / g * inv_u0 % u1;

    let mut r = r0 + x * m0;

    let lcm = m0 * u1;

    if r < 0 {
        r += lcm;
    }

    debug_assert!(r1 <= r && r < lcm);

    Some((lcm, r))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::least_common_multiple_with_gcd_i64::lcm;

        let cases = vec![
            ((10, 3, 14, 7), 63),
            ((10, 3, 14, 6), -1),
            ((15, 2, 17, 8), 212),
            ((5, 3, 7, 4), 18),
            ((1, 0, 2, 0), 0),
            ((2, 0, 1, 0), 0),
        ];

        for ((m0, r0, m1, r1), r) in cases {
            let ans = if r < 0 { None } else { Some((lcm(m0, m1), r as i64)) };

            assert_eq!(safe_crt(m0, r0, m1, r1), ans);
        }
    }
}
