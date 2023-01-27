use crate::prime_factorize_with_least_prime_factor_table_usize::*;

pub fn is_pairwise_coprime(a: &[usize]) -> bool {
    let k = a.iter().max().unwrap() + 1;

    let mut cnt = vec![0; k];

    let f = PrimeFactorize::new(k);

    for &x in a.iter() {
        for (p, _) in f.factorize(x) {
            cnt[p] += 1;

            if cnt[p] > 1 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert!(is_pairwise_coprime(&[3, 4, 5]));
    }
}
