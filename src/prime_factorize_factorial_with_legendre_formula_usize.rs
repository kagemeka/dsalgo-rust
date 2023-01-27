use crate::{
    legendre_formula_u64::legendre,
    sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes,
};

pub fn factorize_factorial(n: usize) -> Vec<(usize, usize)> {
    enumerate_primes(n as u32 + 1)
        .into_iter()
        .map(|p| (p as usize, legendre(n as u64, p as u64) as usize))
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            factorize_factorial(100),
            [
                (2, 97,),
                (3, 48,),
                (5, 24,),
                (7, 16,),
                (11, 9,),
                (13, 7,),
                (17, 5,),
                (19, 5,),
                (23, 4,),
                (29, 3,),
                (31, 3,),
                (37, 2,),
                (41, 2,),
                (43, 2,),
                (47, 2,),
                (53, 1,),
                (59, 1,),
                (61, 1,),
                (67, 1,),
                (71, 1,),
                (73, 1,),
                (79, 1,),
                (83, 1,),
                (89, 1,),
                (97, 1,),
            ]
        );
    }
}
