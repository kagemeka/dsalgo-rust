use crate::integer_square_root_with_binary_search_u64::isqrt;

/// O(N^{3/4})

pub fn prime_pi_fast(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    let sqrt = isqrt(n) as usize;

    let n = n as usize;

    // consider sieve of Eratosthenes' transitions.
    // S(j, p) := number of trues in [2, j] after sieving with prime p.
    let mut small = vec![0; sqrt + 1]; // small[j] = S(j, p)
    let mut large = vec![0; sqrt + 1];

    // large[k] := S([n/k]=j, p)
    // large[0] is undefined.
    for i in 1..=sqrt {
        small[i] = i - 1;

        large[i] = n / i - 1;
    }

    for i in 2..=sqrt {
        if small[i] == small[i - 1] {
            continue;
            // i is not prime.
        }

        // we want update S(j, i) such that j >= i * i.
        // for j > sqrt(n), update large[inv] such that j = [N/inv].
        // for j <= sqrt(n), update small[j].
        let pi = small[i - 1];

        // S(p - 1, p - 1) = pi(p - 1).
        // compute S(j, i) -= S(j/i, i - 1) - pi
        // for large
        // large[n/j] -= large[n/(j/i)] - pi = large[(n/j)i] - pi
        // large[k] -= large[ki] - pi
        // because j = [N/k] >= i*i, k <= [N/(i*i)]
        // be careful of updating in forward order because of in-place.
        let border = sqrt / i;

        let n_i = n / i; // cache
        for k in 1..=border {
            large[k] -= large[k * i] - pi;
        }

        for k in border + 1..=sqrt.min(n_i / i) {
            large[k] -= small[n_i / k] - pi;
        }

        // for small
        // just small[j] -= small[j/i] - pi (i*i <= j <= sqrt)
        // be careful of updating in reverse order because of in-place.
        // for optimization, use multiplication instead of division
        // by computing giving dp instead of receiving.
        // small[j=[k*i, sqrt]] -= small[k] - pi (i <= k <= sqrt/i)
        for k in (i..=border).rev() {
            let sub = small[k] - pi;

            small[(k * i)..].iter_mut().take(i).for_each(|j| *j -= sub);
        }
    }

    large[1] as u64
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;
        use crate::test_fast_prime_counting::test_fast_prime_counting;

        test_fast_prime_counting(&prime_pi_fast);
    }
}
