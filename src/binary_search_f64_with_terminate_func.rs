pub fn binary_search<F, G>(
    is_ok: F,
    is_done: G,
    max_epoch: usize,
    mut ng: f64,
    mut ok: f64,
) -> f64
where
    F: Fn(f64) -> bool,
    G: Fn(f64, f64) -> bool,
{
    for _ in 0..max_epoch {
        if is_done(ng, ok) {
            break;
        }

        let x = (ng + ok) / 2.;

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
        assert_eq!(
            binary_search(
                |x| x * x >= 1000.,
                |ng, ok| ok - ng < 0.1,
                20,
                0.,
                100.
            ),
            31.640625,
        );

        assert_eq!(
            binary_search(
                |x| x * x <= 1000.,
                |ng, ok| ng - ok < 0.1,
                20,
                100.,
                0.
            ),
            31.54296875,
        );
    }
}
