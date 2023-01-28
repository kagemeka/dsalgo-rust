use crate::rng_static_xorshift64::static_xorshift64;

pub fn random_sequence<T: Clone>(
    pool: &[T],
    size: usize,
) -> Vec<T> {
    let n = pool.len();

    (0..size).map(|_| pool[static_xorshift64() as usize % n].clone()).collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let pool: Vec<_> = (0..1000).collect();

        let res: Vec<_> = (0..10).map(|_| random_sequence(&pool, 10)).collect();

        println!("{:#?}", res);
    }
}
