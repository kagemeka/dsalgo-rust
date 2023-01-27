use crate::extended_euclidean_modular_gcd_inverse_usize_with_extgcd::*;

/// avoid overflows unless lcm(m0, m1) overflows.

pub fn safe_crt(
    mut m0: usize,
    mut r0: usize,
    mut m1: usize,
    mut r1: usize,
) -> Option<(usize, usize)> {
    use std::mem::swap;

    assert!(0 < m0 && 0 < m1);

    r0 %= m0;

    r1 %= m1;

    if m0 < m1 {
        // necessarily to avoid overflow
        swap(&mut r0, &mut r1);

        swap(&mut m0, &mut m1);
    }

    if m0 % m1 == 0 {
        return if r0 % m1 == r1 { Some((m0, r0)) } else { None };
    }

    let (g, inv_u0) = mod_gcd_inv(m1, m0 % m1);

    // let rd = r1.abs_diff(r0); // to avoid overflow in case r1 < r0. v1.60 <=
    let rd = if r1 < r0 { r0 - r1 } else { r1 - r0 };

    if rd % g != 0 {
        return None;
    }

    let u1 = m1 / g;

    let mut x = rd / g * inv_u0 % u1;

    // x <= m1^2. so if m0 < m1, it should be swapped at first.
    if r1 < r0 {
        x = u1 - x;
    }

    let mut r = r0 + x * m0;

    let lcm = m0 * u1;

    if r >= lcm {
        r -= lcm;
    }

    debug_assert!(r1.max(r0) <= r && r < lcm);

    Some((lcm, r))
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

            assert_eq!(safe_crt(m0, r0, m1, r1), ans);
        }
    }
}
