pub fn ad_day(
    mut y: usize,
    mut m: usize,
    d: usize,
) -> usize {
    if m < 3 {
        m += 12;

        y -= 1;
    }

    365 * y + y / 4 - y / 100 + y / 400 + 306 * (m + 1) / 10 + d - 428
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![((1, 1, 1), 1), ((2022, 8, 8), 738375)];

        for ((y, m, d), days) in cases {
            assert_eq!(ad_day(y, m, d), days);
        }
    }
}
