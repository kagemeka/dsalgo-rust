pub fn sum_of_xor(
    modulus: usize,
    a: &[usize],
    max_bit: usize,
) -> usize {
    let n = a.len();

    let mut s = 0;

    for i in 0..max_bit {
        let c: usize = a.iter().map(|&x| x >> i & 1).sum();

        s += (1 << i) % modulus * c % modulus * (n - c) % modulus;

        s %= modulus;
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: usize = 1_000_000_007;

        let cases = vec![(
            vec![
                3, 14, 159, 2653, 58979, 323846, 2643383, 27950288, 419716939,
                9375105820,
            ],
            103715602,
        )];

        for (a, ans) in cases {
            assert_eq!(sum_of_xor(MOD, &a, 60), ans);
        }
    }
}
