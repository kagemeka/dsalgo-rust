use crate::greatest_common_divisor_euclidean_recurse_i64::gcd;

pub fn segment_lattice_points(
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
) -> i64 {
    gcd(x1 - x0, y1 - y0) + 1
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![((0, 1, 6, 9), 3)];

        for ((x0, y0, x1, y1), ans) in cases {
            assert_eq!(segment_lattice_points(x0, y0, x1, y1), ans);
        }
    }
}
