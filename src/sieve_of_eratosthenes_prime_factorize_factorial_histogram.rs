pub fn factorize_factorial(n: usize) -> Vec<usize> {
    let mut e = vec![0; n + 1];

    let mut v: Vec<_> = (0..=n).collect();

    for i in 2..=n {
        if v[i] == 1 {
            continue;
        }

        for j in (i..=n).step_by(i) {
            while v[j] % i == 0 {
                e[i] += 1;

                v[j] /= i;
            }
        }
    }

    e
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(factorize_factorial(10), [0, 0, 8, 4, 0, 2, 0, 1, 0, 0, 0,]);
    }
}
