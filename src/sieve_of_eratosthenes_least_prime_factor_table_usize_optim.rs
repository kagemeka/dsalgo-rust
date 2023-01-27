/// valid for n >= 2. please do not access lpf[0] or lpf[1].

pub fn least_prime_factor(size: usize) -> Vec<usize> {
    let mut lpf: Vec<usize> = (0..size).collect();

    for i in (2..size).take_while(|i| i * i < size) {
        if lpf[i] != i {
            continue;
        }

        for j in (i * i..size).step_by(i) {
            if lpf[j] == j {
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
            [0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19]
        );
    }
}
