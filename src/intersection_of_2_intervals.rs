/// intersection of [l0, r0), [l1, r1)

pub fn intersection(
    l0: i64,
    r0: i64,
    l1: i64,
    r1: i64,
) -> Option<(i64, i64)> {
    let l = l0.max(l1);

    let r = r0.min(r1);

    if r < l {
        None
    } else {
        Some((l, r))
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [
            ((0, 3, 1, 5), Some((1, 3))),
            ((0, 1, 4, 5), None),
            ((0, 3, 3, 7), Some((3, 3))),
        ];

        for ((l0, r0, l1, r1), ans) in cases {
            assert_eq!(intersection(l0, r0, l1, r1), ans);
        }
    }
}
