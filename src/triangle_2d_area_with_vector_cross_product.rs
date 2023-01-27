pub fn triangle_area(
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
) -> f64 {
    (x0 * y1 - x1 * y0) as f64 / 2.0
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [((2, 1, -3, 3), 4.5), ((2, 1, -3, -3), -1.5)];

        for ((x0, y0, x1, y1), ans) in cases {
            assert_eq!(triangle_area(x0, y0, x1, y1), ans);
        }
    }
}
