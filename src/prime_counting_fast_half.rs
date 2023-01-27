//! this does not pass test.

// FIXME: fix bug.
use crate::integer_square_root_with_binary_search_u64::isqrt;

/// O(N^{3/4}) with constant time optimization.
/// insipired by O(N^{3/4}/log{N}) implementation.

pub fn prime_pi_fast_half(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let sqrt = isqrt(n) as usize;

    let n = n as usize;

    let size = (sqrt + 1) >> 1;

    let half = |j: usize| (j - 1) >> 1;

    let mut small = (0..size).collect::<Vec<_>>();

    let mut large =
        (0..size).map(|i| half(n / (i << 1 | 1))).collect::<Vec<_>>();

    for i in (3..=sqrt).step_by(2) {
        let i_half = half(i);

        if small[i_half] == small[i_half - 1] {
            continue;
        }

        let pi = small[half(i - 1)];

        let mut border = sqrt / i;

        if border & 1 == 0 {
            border -= 1;
        }

        let n_i = n / i;

        for k in (1..=border).step_by(2) {
            // debug!(large[half(k)], large[half(k * i)], pi);
            large[half(k)] -= large[half(k * i + 1)].saturating_sub(pi);
            // large[half(k)] += pi;
            // large[half(k)] -= large[half(k * i + 1)];
        }

        for k in (border + 2..=sqrt.min(n_i / i)).step_by(2) {
            large[half(k)] -= small[half(n_i / k)] - pi;
        }

        for k in (i..=border).rev().step_by(2) {
            let sub = small[half(k)] - pi;

            small[half(k * i)..]
                .iter_mut()
                .take(half(i) + 1)
                .for_each(|j| *j -= sub);
        }
    }

    large[0] as u64 + 1
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;
        // use crate::test_fast_prime_counting::test_fast_prime_counting;
        // test_fast_prime_counting(&prime_pi_fast_half);
    }
}
