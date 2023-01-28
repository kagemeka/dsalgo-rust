pub fn chebychev_dist(
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
) -> i64 {
    (x1 - x0).abs().max((y1 - y0).abs())
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(chebychev_dist(-1, 3, 5, 0), 6);
    }
}
