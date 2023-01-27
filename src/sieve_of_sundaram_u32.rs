//! sieve of sundaram

pub fn enumerate_primes(less_than: u32) -> Vec<u32> {
    let mut a = vec![];

    if less_than <= 2 {
        return a;
    }

    a.push(2);

    let n = (less_than >> 1) as usize;

    let mut is_p = vec![true; n];

    for i in 1..n {
        if is_p[i] {
            a.push(((i as u32) << 1) | 1);
        }

        for j in (i * (i + 1) << 1..n).step_by((i << 1) | 1) {
            is_p[j] = false;
        }
    }

    a
}

#[cfg(test)]

mod tests {

    use super::enumerate_primes as sundaram;

    #[test]

    fn test() {
        use crate::sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes;

        let lim = [99, 100, 101, 102, 1 << 17];

        for l in lim {
            assert_eq!(sundaram(l), enumerate_primes(l));
        }
    }
}
