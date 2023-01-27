/// intersection of [l0, r0), [l1, r1)

pub fn intersection(
    l0: i64,
    r0: i64,
    l1: i64,
    r1: i64,
) -> i64 {
    0.max(r0.min(r1) - l0.max(l1))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [((0, 3, 1, 5), 2), ((0, 1, 4, 5), 0), ((0, 3, 3, 7), 0)];

        for ((l0, r0, l1, r1), ans) in cases {
            assert_eq!(intersection(l0, r0, l1, r1), ans);
        }
    }
}
