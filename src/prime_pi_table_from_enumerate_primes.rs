use crate::sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes;

pub fn prime_pi_table(size: usize) -> Vec<u32> {
    let mut pi = vec![0; size];

    for p in enumerate_primes(size as u32) {
        pi[p as usize] = 1;
    }

    for i in 0..size - 1 {
        pi[i + 1] += pi[i];
    }

    pi
}
