use crate::prime_factorize_with_least_prime_factor_table_u32::PrimeFactorize;

pub fn prime_factorize_factorial_lpf(n: u32) -> Vec<u32> {
    let size = n as usize + 1;

    let mut count = vec![0; size];

    let lpf = PrimeFactorize::new(size);

    for i in 2..=n as u32 {
        for (p, e) in lpf.factorize(i) {
            count[p as usize] += e;
        }
    }

    count
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
