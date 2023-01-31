use crate::{
    greatest_common_divisor_euclidean_recurse_i64::gcd,
    modular_inverse_euler_theorem_i64::modinv,
};

/// find a solution for ax = b (mod m)

pub fn mod_linear_equation(
    mut m: i64,
    mut a: i64,
    mut b: i64,
) -> Option<i64> {
    let g = gcd(m, a);

    if b % g != 0 {
        return None;
    }

    m /= g;

    a /= g;

    b /= g;

    let x = b * modinv(m, a) % m;

    Some((x + m) % m)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/abc186/tasks/abc186_e
        let cases = vec![
            ((10, 4, 3), 2),
            ((1000, 11, 2), -1),
            ((998244353, 897581057, 595591169), 249561088),
            ((10000, 6, 14), 3571),
        ];

        for ((n, s, k), ans) in cases {
            let ans = if ans == -1 { None } else { Some(ans) };

            assert_eq!(mod_linear_equation(n, k, -s), ans);
        }
    }
}
