use crate::integer_square_root_with_binary_search_u64::isqrt;

/// Compute \pi(n)
/// O(N^{3/4}/log{N})
/// reference
/// - https://judge.yosupo.jp/submission/61553

pub fn prime_pi_fast_optimized(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let half = |i: usize| (i - 1) >> 1;

    let sqrt = isqrt(n) as usize;

    let n = n as usize;

    let mut size = (sqrt + 1) >> 1;

    // for memory saving. do not have space for even numbers.
    let mut small: Vec<usize> = (0..size).collect();

    // j=1, 3, 5, 7, ..., k=0, 1, 2, 3
    // -> unsieved count less than or equal to j is (j - 1) >> 1 = k.
    let mut large: Vec<usize> =
        (0..size).map(|i| half(n / (i << 1 | 1))).collect();

    // (j - 1) >> 1 = k <-> (k << 1 | 1) = j
    let mut unsieved_nums: Vec<usize> = (0..size).map(|i| i << 1 | 1).collect();

    // 1initially, 1, 3, 5, ... (odd at most sqrt(n))
    // unsieved_nums[..size] are odd integers which are still unsieved.
    // (size will be updated in each iteration)
    // unsieved_nums[size..] are no longer used.
    let mut checked_or_sieved = vec![false; size];

    // 1, 2 -> 0, 3, 4 -> 1, ... (because even numbers are skipped.)
    let mut pi = 0;

    for i in (3..=sqrt).step_by(2) {
        if checked_or_sieved[half(i)] {
            // sieved
            continue;
        }

        let i2 = i * i;

        if i2 * i2 > n {
            break;
        }

        checked_or_sieved[half(i)] = true; // checked
        for j in (i2..=sqrt).step_by(i << 1) {
            checked_or_sieved[half(j)] = true;
        }

        // update large and unsieved_nums
        let mut ptr = 0;

        for k in 0..size {
            let j = unsieved_nums[k];

            if checked_or_sieved[half(j)] {
                continue;
            }

            let border = j * i;

            large[ptr] = large[k]
                - if border <= sqrt {
                    large[small[border >> 1] - pi]
                } else {
                    small[half(n / border)]
                }
                + pi;

            unsieved_nums[ptr] = j;

            ptr += 1;
        }

        size = ptr;

        let mut j = half(sqrt);

        let mut k = sqrt / i - 1 | 1;

        while k >= i {
            let c = small[k >> 1] - pi;

            let e = k * i >> 1;

            while j >= e {
                small[j] -= c;

                j -= 1;
            }

            k -= 2;
        }

        pi += 1;
    }

    // be careful of overflow.
    large[0] += if pi > 0 {
        size + ((pi - 1) << 1)
    } else {
        // -1 << 1 == -2
        size.saturating_sub(2)
        // if size == 1,
        // (size + ((pi - 1) << 1)) * (size - 1) >> 1 == 0
        // regardless of `size + ((pi - 1) << 1)`
    } * (size - 1)
        >> 1;

    for k in 1..size {
        large[0] -= large[k];
    }

    for k in 1..size {
        let q = unsieved_nums[k];

        let n_q = n / q;

        let e = small[half(n_q / q)] - pi;

        if e < k + 1 {
            break;
        }

        let mut t = 0;

        for l in k + 1..=e {
            t += small[half(n_q / unsieved_nums[l])];
        }

        large[0] += t - (e - k) * (pi + k - 1);
    }

    large[0] as u64 + 1
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;
        use crate::test_fast_prime_counting::test_fast_prime_counting;

        test_fast_prime_counting(&prime_pi_fast_optimized);
    }
}
