use crate::sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes;

pub fn is_prime(size: usize) -> Vec<bool> {
    let mut is_prime = vec![false; size];

    for p in enumerate_primes(size as u32) {
        is_prime[p as usize] = true;
    }

    is_prime
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let is_prime = is_prime(20);

        assert_eq!(
            is_prime,
            vec![
                false, false, true, true, false, true, false, true, false,
                false, false, true, false, true, false, false, false, true,
                false, true
            ],
        );
    }
}
