//! for sequence, see crate::binary_search_on_sequence.

pub fn binary_search<F>(
    is_ok: F,
    mut ng: i64,
    mut ok: i64,
) -> i64
where
    F: Fn(i64) -> bool,
{
    while ok.abs_diff(ng) > 1 {
        let x = (ng + ok) >> 1;

        if is_ok(x) {
            ok = x;
        } else {
            ng = x;
        }
    }

    ok
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(binary_search(|x| x * x >= 1000, 0, 100), 32);

        assert_eq!(binary_search(|x| x * x <= 1000, 100, 0), 31);
    }
}
