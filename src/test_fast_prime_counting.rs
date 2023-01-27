//! internal test module.
//! commonly used for testing fast prime counting algorithms.
//! which can compute pi(N <= 10^10) in approximately 1.0 sec.

use crate::{
    prime_pi_power_of_10::PRIME_PI_POWER_OF_10,
    prime_pi_table_from_enumerate_primes::prime_pi_table,
};

#[allow(dead_code)]

pub(crate) fn test_fast_prime_counting<F>(pi: &F)
where
    F: Fn(u64) -> u64,
{
    const N: u32 = 1 << 10;

    let ans = prime_pi_table(N as usize);

    for i in 0..N {
        let res = pi(i as u64) as u32;

        assert_eq!(ans[i as usize], res);
    }

    for i in 0..11 {
        let ans = PRIME_PI_POWER_OF_10[i as usize] as u64;

        let res = pi(10u64.pow(i as u32));

        assert_eq!(ans, res);
    }
}
