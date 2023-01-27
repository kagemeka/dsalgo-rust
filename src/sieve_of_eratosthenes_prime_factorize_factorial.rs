pub fn factorize_factorial(n: usize) -> Vec<(usize, usize)> {
    let mut factors = vec![];

    let mut v: Vec<_> = (0..=n).collect();

    for i in 2..=n {
        if v[i] == 1 {
            continue;
        }

        let mut e = 0;

        for j in (i..=n).step_by(i) {
            while v[j] % i == 0 {
                e += 1;

                v[j] /= i;
            }
        }

        factors.push((i, e));
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
            [(2, 8,), (3, 4,), (5, 2,), (7, 1,),]
        );
    }
}
