pub fn greatest_prime_factor(size: usize) -> Vec<usize> {
    let mut gpf: Vec<usize> = vec![0; size];

    for i in 2..size {
        if gpf[i] != 0 {
            continue;
        }

        for j in (i..size).step_by(i) {
            gpf[j] = i;
        }
    }

    gpf
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            greatest_prime_factor(20),
            [0, 0, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5, 2, 17, 3, 19]
        );
    }
}
