pub fn factorize_factorial(n: usize) -> Vec<usize> {
    let mut factors = vec![];

    let mut v: Vec<_> = (0..=n).collect();

    for i in 2..=n {
        if v[i] == 1 {
            continue;
        }

        for j in (i..=n).step_by(i) {
            while v[j] % i == 0 {
                factors.push(i);

                v[j] /= i;
            }
        }
    }

    factors
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            factorize_factorial(10),
            [2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 5, 5, 7]
        );
    }
}
