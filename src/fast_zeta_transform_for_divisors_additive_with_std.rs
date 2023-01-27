use std::ops::*;

use crate::sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes;

pub fn fast_zeta_divisors<T: Clone + Add<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let n = f.len();

    for p in enumerate_primes(n) {
        for i in 1..(n - 1) / p + 1 {
            f[i * p] = f[i * p].clone() + f[i].clone();
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::number_of_divisors_table_naive_usize::number_of_divisors;

        let n = 1 << 15;

        let mut a = vec![1; n];

        a[0] = 0;

        let res = fast_zeta_divisors(a);

        assert_eq!(res, number_of_divisors(n));
    }
}
