pub fn chebychev_dist(
    p0: &[i64],
    p1: &[i64],
) -> i64 {
    let n = p0.len();

    assert_eq!(p1.len(), n);

    p0.iter().zip(p1.iter()).map(|(&p, &q)| (p - q).abs()).max().unwrap()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(chebychev_dist(&[-1, 3], &[5, 0]), 6);
    }
}
