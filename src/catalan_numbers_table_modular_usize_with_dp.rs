pub fn catalan_numbers(
    m: usize,
    size: usize,
) -> Vec<usize> {
    let mut c = vec![0; size];

    c[0] = 1;

    c[1] = 1;

    for i in 2..size {
        for j in 0..i {
            c[i] += c[j] * c[i - 1 - j];

            c[i] %= m;
        }
    }

    c
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::catalan_numbers_constant::*;

        let m = 998_244_353;

        let mut n = 0;

        while CATALAN_NUMBERS[n] < m {
            n += 1;
        }

        assert_eq!(catalan_numbers(m, n), CATALAN_NUMBERS[..n]);
    }
}
