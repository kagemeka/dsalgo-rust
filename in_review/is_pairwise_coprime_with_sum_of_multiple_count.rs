pub fn is_pairwise_coprime(a: &[usize]) -> bool {
    let k = a.iter().max().unwrap() + 1;

    let mut cnt = vec![0; k];

    for &x in a.iter() {
        cnt[x] += 1;
    }

    let mut is_prime = vec![true; k];

    for i in 2..k {
        if !is_prime[i] {
            continue;
        }

        let mut c = cnt[i];

        for j in (i << 1..k).step_by(i) {
            is_prime[j] = false;

            c += cnt[j];
        }

        if c > 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert!(is_pairwise_coprime(&[3, 4, 5]));
    }
}
