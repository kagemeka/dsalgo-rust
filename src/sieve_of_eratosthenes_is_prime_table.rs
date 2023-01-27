pub fn is_prime(size: usize) -> Vec<bool> {
    let mut is_prime = vec![true; size];

    is_prime[0] = false;

    is_prime[1] = false;

    for i in (2..size).take_while(|i| i * i < size) {
        if !is_prime[i] {
            continue;
        }

        for j in (i * i..size).step_by(i) {
            is_prime[j] = false;
        }
    }

    is_prime
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::sieve_of_eratosthenes_enumerate_primes_u32::*;

        let k: usize = 10000;

        let mut is_p = vec![false; k];

        for p in enumerate_primes(k as u32) {
            is_p[p as usize] = true;
        }

        assert_eq!(is_prime(k), is_p);
    }
}
