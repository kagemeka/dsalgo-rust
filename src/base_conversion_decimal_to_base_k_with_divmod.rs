use crate::divmod::divmod;

pub fn decimal_to_base_k(
    k: i64,
    mut n: i64,
) -> Vec<i64> {
    assert!(k.abs() >= 2);

    let mut nums = vec![];

    loop {
        let (q, mut r) = divmod(n, k);

        n = q;

        if r < 0 {
            r -= k;

            n += 1;
        }

        nums.push(r);

        if n == 0 {
            return nums;
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((2, 6), vec![0, 1, 1]),
            ((2, 10), vec![0, 1, 0, 1]),
            ((-2, 10), vec![0, 1, 1, 1, 1]),
        ];

        for ((k, n), ans) in cases {
            assert_eq!(decimal_to_base_k(k, n), ans);
        }
    }
}
