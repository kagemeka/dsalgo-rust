pub fn number_of_prime_factors(size: usize) -> Vec<usize> {
    let mut f = vec![0; size];

    for i in 2..size {
        if f[i] != 0 {
            continue;
        }

        for j in (i..size).step_by(i) {
            f[j] += 1;
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const ANS: &[usize] = &[
            0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2,
            1, 2, 1, 2, 1, 2, 1, 3, 1, 1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 3, 1, 2,
            2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 3,
            1, 2, 2, 3, 1, 2, 1, 2, 2, 2, 2, 3, 1, 2, 1, 2, 1, 3, 2, 2, 2, 2,
            1, 3, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 1, 3, 1, 2, 3, 2, 1, 2, 1, 3,
            2,
        ];

        let n = ANS.len();

        assert_eq!(&number_of_prime_factors(n + 1)[1..], ANS);
    }
}
