use crate::extended_euclidean_gcd_i64_recurse::extgcd;

pub fn crt(
    m0: i64,
    mut r0: i64,
    m1: i64,
    mut r1: i64,
) -> Option<(i64, i64)> {
    assert!(m0 > 0 && m1 > 0);

    r0 %= m0;

    r1 %= m1;

    let dr = r1 - r0;

    let (g, p, _) = extgcd(m0, m1);

    if dr % g != 0 {
        return None;
    }

    let lcm = m0 / g * m1;

    let r = (r0 + dr / g * p as i64 % lcm * m0) % lcm;

    Some((lcm, (r + lcm) % lcm))
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

            assert_eq!(crt(m0, r0, m1, r1), ans);
        }
    }
}
