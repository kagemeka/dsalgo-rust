pub fn enumerate_primes(less_than: u32) -> Vec<u32> {
    let n = less_than as usize;

    let mut primes = Vec::with_capacity(n >> 4);

    if n > 2 {
        primes.push(2);
    }

    let mut is_prime = vec![true; n >> 1];

    for i in (3..n).step_by(2) {
        if !is_prime[i >> 1] {
            continue;
        }

        primes.push(i as u32);

        for j in (i * i..n).step_by(i << 1) {
            is_prime[j >> 1] = false;
        }
    }

    primes
}

#[test]

fn test() {
    assert_eq!(
        enumerate_primes(50),
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47],
    )
}
