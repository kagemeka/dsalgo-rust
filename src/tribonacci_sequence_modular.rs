pub fn tribonacci(
    size: usize,
    modulus: usize,
) -> Vec<usize> {
    let mut t = vec![0; size];

    t[2] = 1;

    for i in 3..size {
        t[i] = (t[i - 1] + t[i - 2] + t[i - 3]) % modulus;
    }

    t
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: usize = 1000000007;

        let n = 100000;

        let t = tribonacci(n, MOD);

        assert_eq!(t[..10].to_vec(), [0, 0, 1, 1, 2, 4, 7, 13, 24, 44,]);
    }
}
