pub fn factorize_factorial(n: usize) -> Vec<usize> {
    let mut e = vec![0; n + 1];

    for mut x in 2..n + 1 {
        for i in 2..x {
            if i * i > x {
                break;
            }

            while x % i == 0 {
                x /= i;

                e[i] += 1;
            }
        }

        if x > 1 {
            e[x] += 1;
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
