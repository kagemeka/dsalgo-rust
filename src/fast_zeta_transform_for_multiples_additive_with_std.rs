use std::ops::*;

use crate::sieve_of_eratosthenes_enumerate_primes_usize::enumerate_primes;

pub fn fast_zeta_multiples<T: Clone + Add<Output = T>>(
    mut f: Vec<T>
) -> Vec<T> {
    let n = f.len();

    for p in enumerate_primes(n) {
        for i in (1..(n - 1) / p + 1).rev() {
            f[i] = f[i].clone() + f[i * p].clone();
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::number_of_multiples_table_with_multiple_zeta_usize::*;

        let n = 1 << 15;

        let mut a = vec![1; n];

        a[0] = 0;

        let res = fast_zeta_multiples(a);

        assert_eq!(res, number_of_multiples(n));
    }
}
