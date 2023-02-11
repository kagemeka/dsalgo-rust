use crate::sieve_of_eratosthenes_least_prime_factor_table_usize_optim2::*;

pub fn greatest_prime_factor(size: usize) -> Vec<usize> {
    let mut gpf: Vec<usize> = least_prime_factor(size);

    for i in 2..size {
        if gpf[i] != i {
            gpf[i] = gpf[i / gpf[i]];
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
            [0, 1, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5, 2, 17, 3, 19]
        );
    }
}
