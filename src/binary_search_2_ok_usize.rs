//! for sequence, see crate::binary_search_on_sequence.

pub fn binary_search<F>(
    is_ok: F,
    mut lo_ok: usize,
    mut hi_ok: usize,
) -> usize
where
    F: Fn(usize) -> bool,
{
    while lo_ok != hi_ok {
        let x = (lo_ok + hi_ok) >> 1;

        if is_ok(x) {
            hi_ok = x;
        } else {
            lo_ok = x + 1;
        }
    }

    lo_ok
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(binary_search(|x| x * x >= 1000, 0, 100), 32);
    }
}
