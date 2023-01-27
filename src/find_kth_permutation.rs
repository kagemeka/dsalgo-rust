use crate::find_kth_set_bit_by_removing_lsb::kth_set_bit;

pub fn kth_permutation(
    n: usize,
    mut k: usize,
) -> Vec<usize> {
    let mut s = (1 << n) - 1;

    let mut p = vec![0; n];

    let mut fact = vec![0; n];

    fact[0] = 1;

    for i in 0..n - 1 {
        fact[i + 1] = (i + 1) * fact[i];
    }

    for i in 0..n {
        let j = kth_set_bit(s, k / fact[n - 1 - i]) as usize;

        k %= fact[n - 1 - i];

        p[i] = j;

        s &= !(1 << j);
    }

    p
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let p = kth_permutation(10, 1_000_000 - 1);

        assert_eq!(p, [2, 7, 8, 3, 9, 1, 5, 4, 6, 0]);
        // testcase source: https://projecteuler.net/problem=24
    }
}
