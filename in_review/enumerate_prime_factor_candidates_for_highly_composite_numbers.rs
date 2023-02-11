use crate::sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes;

pub fn enumerate_prime_candidates(n: u64) -> Vec<u64> {
    const MAX_P: u32 = 1 << 10;

    let primes = enumerate_primes(MAX_P);

    let mut cands = vec![];

    let mut prod = 1u64;

    for p in primes {
        let p = p as u64;

        if let Some(next) = prod.checked_mul(p) {
            if next > n {
                break;
            }

            prod = next;
        } else {
            break;
        }

        cands.push(p);
    }

    cands
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            enumerate_prime_candidates(10u64.pow(18)),
            [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,]
        );
    }
}
