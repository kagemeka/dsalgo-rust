use crate::sieve_of_eratosthenes_enumerate_primes_u32::enumerate_primes;

pub fn least_prime_factor(size: usize) -> Vec<Option<u32>> {
    let mut lpf = vec![None; size];

    for p in enumerate_primes(size as u32) {
        debug_assert!(lpf[p as usize].is_none());

        lpf[p as usize] = Some(p);
    }

    for i in (4..size).step_by(2) {
        lpf[i] = Some(2);
    }

    for i in (3..size).step_by(2).take_while(|&i| i * i < size) {
        debug_assert!(lpf[i].is_some());

        if lpf[i] != Some(i as u32) {
            continue;
        }

        for j in (i * i..size).step_by(i << 1) {
            if let Some(x) = lpf[j] {
                debug_assert!(x < i as u32);
            } else {
                lpf[j] = Some(i as u32);
            }
        }
    }

    lpf
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let lpf = least_prime_factor(1 << 10);

        assert_eq!(
            lpf.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                None,
                None,
                Some(2),
                Some(3),
                Some(2),
                Some(5),
                Some(2),
                Some(7),
                Some(2),
                Some(3)
            ],
        );
    }
}
