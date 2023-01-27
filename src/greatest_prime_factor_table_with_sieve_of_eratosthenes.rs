use crate::least_prime_factor_table_with_sieve_of_eratosthenes_u32::*;

pub fn greatest_prime_factor(size: usize) -> Vec<Option<u32>> {
    let lpf = least_prime_factor(size);

    let mut gpf = vec![None; size];

    for i in 2..size {
        gpf[i] = if lpf[i] == Some(i as u32) {
            lpf[i]
        } else {
            gpf[i / lpf[i].unwrap() as usize]
        }
    }

    gpf
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let gpf = greatest_prime_factor(100);

        assert_eq!(gpf[51], Some(17));

        assert_eq!(
            gpf.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                None,
                None,
                Some(2),
                Some(3),
                Some(2),
                Some(5),
                Some(3),
                Some(7),
                Some(2),
                Some(3),
            ],
        );
    }
}
