use crate::dist_2d_to_the_power_of_2::dist2;

pub fn check_intersect(
    x0: i64,
    y0: i64,
    r0: i64,
    x1: i64,
    y1: i64,
    r1: i64,
) -> bool {
    let d2 = dist2(x0, y0, x1, y1);

    d2 <= (r0 + r1).pow(2) && d2 >= (r0 - r1).pow(2)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((0, 0, 1, 0, 0, 2), false),
            ((0, 0, 2, 3, 3, 3), true),
            ((2, 3, 1, 2, 0, 2), true),
        ];

        for ((x0, y0, r0, x1, y1, r1), ans) in cases {
            assert_eq!(check_intersect(x0, y0, r0, x1, y1, r1), ans);
        }
    }
}
