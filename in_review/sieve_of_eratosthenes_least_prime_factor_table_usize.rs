/// valid for n >= 2. please do not access lpf[0] or lpf[1].

pub fn least_prime_factor(size: usize) -> Vec<usize> {
    let mut lpf = vec![0; size];

    for i in 2..size {
        if lpf[i] != 0 {
            continue;
        }

        for j in (i..size).step_by(i) {
            if lpf[j] == 0 {
                lpf[j] = i;
            }
        }
    }

    lpf
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            least_prime_factor(20),
            [0, 0, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19]
        );
    }
}
