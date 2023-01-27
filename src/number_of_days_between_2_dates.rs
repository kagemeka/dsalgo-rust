use crate::fairfield_ad_day_formula::ad_day;

pub fn number_of_days(
    y0: usize,
    m0: usize,
    d0: usize,
    y1: usize,
    m1: usize,
    d1: usize,
) -> usize {
    ad_day(y1, m1, d1) - ad_day(y0, m0, d0)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((1988, 7, 3, 2014, 5, 17), 9449),
            ((1, 1, 1, 2014, 5, 17), 735369),
            ((2012, 2, 29, 2014, 5, 17), 808),
        ];

        for ((y0, m0, d0, y1, m1, d1), ans) in cases {
            assert_eq!(number_of_days(y0, m0, d0, y1, m1, d1), ans);
        }
    }
}
