/// compute least prime factor table and prime numbers list.
/// O(N)

pub fn linear_sieve(size: usize) -> (Vec<Option<u32>>, Vec<u32>) {
    let mut lpf = vec![None; size];

    let mut a = Vec::with_capacity(size >> 4);

    for i in 2..size {
        if lpf[i].is_none() {
            lpf[i] = Some(i as u32);

            a.push(i as u32);
        }

        for &p in &a {
            if p > lpf[i].unwrap() || p as usize * i >= size {
                break;
            }

            debug_assert!(lpf[p as usize * i].is_none());

            lpf[p as usize * i] = Some(p);
        }
    }

    (lpf, a)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::{
            least_prime_factor_table_with_sieve_of_eratosthenes_u32::*,
            sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes,
        };

        const K: usize = 1 << 10;

        let lpf_ans = least_prime_factor(K);

        let p_ans = enumerate_primes(K as u32);

        assert_eq!((lpf_ans, p_ans), linear_sieve(K));
    }
}
