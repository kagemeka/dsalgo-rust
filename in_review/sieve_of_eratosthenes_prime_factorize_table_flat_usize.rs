pub fn prime_factorize(size: usize) -> Vec<Vec<usize>> {
    let mut factors = vec![vec![]; size];

    let mut v: Vec<_> = (0..size).collect();

    for i in 2..size {
        if !factors[i].is_empty() {
            continue;
        }

        for j in (i..size).step_by(i) {
            while v[j] % i == 0 {
                factors[j].push(i);

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
        let a = prime_factorize(20);

        assert_eq!(
            a,
            vec![
                vec![],
                vec![],
                vec![2,],
                vec![3,],
                vec![2, 2,],
                vec![5,],
                vec![2, 3,],
                vec![7,],
                vec![2, 2, 2,],
                vec![3, 3,],
                vec![2, 5,],
                vec![11,],
                vec![2, 2, 3,],
                vec![13,],
                vec![2, 7,],
                vec![3, 5,],
                vec![2, 2, 2, 2,],
                vec![17,],
                vec![2, 3, 3,],
                vec![19,],
            ]
        );
    }
}
