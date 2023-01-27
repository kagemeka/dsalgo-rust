pub fn mobius(mut n: usize) -> isize {
    let mut k: isize = 0;

    for i in 2..n {
        if i * i > n {
            break;
        }

        let mut e = 0;

        while n % i == 0 {
            e += 1;

            n /= i;
        }

        if e >= 2 {
            return 0;
        }

        k += e;
    }

    if n > 1 {
        k += 1;
    }

    1 - (k & 1) * 2
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MU: &[isize] = &[
            1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0,
            1, 1, -1, 0, 0, 1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0,
            -1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1, 0, -1, 0, 1, 0, 1, 1, -1, 0,
            -1, 1, 0, 0, 1, -1, -1, 0, 1, -1, -1, 0, -1, 1, 0, 0, 1, -1,
        ];

        let n = MU.len();

        for i in 0..n {
            assert_eq!(mobius(i + 1), MU[i]);
        }
    }
}
