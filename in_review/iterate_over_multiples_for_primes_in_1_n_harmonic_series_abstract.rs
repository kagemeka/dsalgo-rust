pub fn iter_over_multiples<F>(
    n: usize,
    mut f: F,
) where
    F: FnMut(usize, usize),
{
    let mut is_prime = vec![true; n >> 1];

    for j in (2..n).step_by(2) {
        f(2, j);
    }

    for i in (3..n).step_by(2) {
        if !is_prime[i >> 1] {
            continue;
        }

        for j in (i..n).step_by(i) {
            f(i, j);

            if j > i {
                is_prime[j >> 1] = false;
            }
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 10;

        let mut is_prime = vec![true; n];

        is_prime[0] = false;

        is_prime[1] = false;

        iter_over_multiples(n, |i, j| {
            if is_prime[i] && j > i {
                is_prime[j] = false;
            }
        });

        assert_eq!(
            is_prime,
            vec![
                false, false, true, true, false, true, false, true, false,
                false
            ]
        );
    }
}
