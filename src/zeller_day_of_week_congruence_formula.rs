/// 0: Sun ~ 6: Sat
/// 0001-1-1 <= date <= 1582-10-4 -> julian calendar
/// 1582-10-5 <= date <= 1582-10-14 -> undefined
/// 1582-10-15 <= date -> gregorian calendar

pub fn day_of_week(
    mut y: usize,
    mut m: usize,
    d: usize,
) -> usize {
    const JULIAN_LAST: (usize, usize, usize) = (1582, 10, 4);

    const GREGORIAN_START: (usize, usize, usize) = (1582, 10, 15);

    let date = (y, m, d);

    assert!(date <= JULIAN_LAST || GREGORIAN_START <= date);

    // assure not in the lost 10 days.
    if m < 3 {
        m += 12;

        y -= 1;
    }

    let c = y / 100;

    y %= 100;

    let gamma = if date <= JULIAN_LAST { 6 * c + 5 } else { 5 * c + c / 4 };

    (d + 13 * (m + 1) / 5 + y + y / 4 + gamma + 6) % 7
}

pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((1, 1, 1), 6),
            ((1582, 10, 15), 5),
            ((1582, 10, 4), 4),
            ((2022, 8, 8), 1),
        ];

        for ((y, m, d), ans) in cases {
            assert_eq!(day_of_week(y, m, d), ans);
        }
    }
}
