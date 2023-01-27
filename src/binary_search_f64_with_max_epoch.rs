pub fn binary_search<F>(
    is_ok: F,
    max_epoch: usize,
    mut ng: f64,
    mut ok: f64,
) -> f64
where
    F: Fn(f64) -> bool,
{
    for _ in 0..max_epoch {
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
            binary_search(|x| x * x >= 1000., 20, 0., 100.),
            31.622791290283203,
        );

        assert_eq!(
            binary_search(|x| x * x <= 1000., 20, 100., 0.),
            31.622695922851563,
        );
    }
}
