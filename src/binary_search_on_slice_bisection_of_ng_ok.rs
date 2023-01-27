pub fn binary_search<T, F>(
    is_ok: F,
    monotonic_sequence: &[T],
) -> usize
where
    F: Fn(&T) -> bool,
{
    let mut ng = -1;

    let mut ok = monotonic_sequence.len() as isize;

    while ok - ng > 1 {
        let i = (ng + ok) >> 1;

        if is_ok(&monotonic_sequence[i as usize]) {
            ok = i;
        } else {
            ng = i;
        }
    }

    ok as usize
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
