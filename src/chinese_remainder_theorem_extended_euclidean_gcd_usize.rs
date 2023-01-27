use crate::extended_euclidean_gcd_i64_recurse::extgcd;

pub fn crt(
    mut m0: usize,
    mut r0: usize,
    mut m1: usize,
    mut r1: usize,
) -> Option<(usize, usize)> {
    use std::mem::swap;

    assert!(m0 > 0 && m1 > 0);

    r0 %= m0;

    r1 %= m1;

    if r0 > r1 {
        swap(&mut r0, &mut r1);

        swap(&mut m0, &mut m1);
    }

    let dr = r1 - r0;

    let (g, mut p, _) = extgcd(m0 as i64, m1 as i64);

    // x = r_0 + pm_0 = r_1 + qm_1
    // g := gcd(m_0, m_1)
    // p'm_0 - q'm_1 = g
    // p = p' * (r_1 - r_0) / g
    let g = g as usize;

    if dr % g != 0 {
        return None;
    }

    let lcm = m0 / g * m1;

    if p < 0 {
        p += lcm as i64;
    }

    Some((lcm, (dr / g * p as usize % lcm * m0 + r0) % lcm))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::least_common_multiple_with_gcd_usize::lcm;

        let cases = vec![
            ((10, 3, 14, 7), 63),
            ((10, 3, 14, 6), -1),
            ((15, 2, 17, 8), 212),
            ((5, 3, 7, 4), 18),
            ((1, 0, 2, 0), 0),
            ((2, 0, 1, 0), 0),
        ];

        for ((m0, r0, m1, r1), r) in cases {
            let ans =
                if r < 0 { None } else { Some((lcm(m0, m1), r as usize)) };

            assert_eq!(crt(m0, r0, m1, r1), ans);
        }
    }
}
