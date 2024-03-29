pub fn binary_search<T, F>(is_ok: F, monotonic_sequence: &[T]) -> usize
where
    F: Fn(&T) -> bool,
{
    let mut lo_ok = 0;

    let mut hi_ok = monotonic_sequence.len();

    while lo_ok < hi_ok {
        let i = (lo_ok + hi_ok - 1) >> 1;

        if is_ok(&monotonic_sequence[i]) {
            hi_ok = i;
        } else {
            lo_ok = i + 1;
        }
    }

    hi_ok
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let v = (0..10).collect::<Vec<_>>();

        assert_eq!(binary_search(&|x: &i32| x >= &5, &v), 5);

        assert_eq!(binary_search(&|x: &i32| x >= &10, &v), 10);

        assert_eq!(binary_search(&|x: &i32| x >= &11, &v), 10);
    }
}
