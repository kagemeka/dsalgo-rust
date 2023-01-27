pub fn base_k_to_decimal(
    k: i64,
    nums: &[i64],
) -> i64 {
    assert!(k.abs() >= 2);

    let mut p = 1;

    let mut n = 0;

    for x in nums {
        n += x * p;

        p *= k;
    }

    n
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((2, vec![0, 1, 1]), 6),
            ((2, vec![0, 1, 0, 1]), 10),
            ((-2, vec![0, 1, 1, 1, 1]), 10),
        ];

        for ((k, nums), ans) in cases {
            assert_eq!(base_k_to_decimal(k, &nums), ans);
        }
    }
}
