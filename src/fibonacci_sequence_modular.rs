pub fn fibonacci(
    size: usize,
    modulus: usize,
) -> Vec<usize> {
    let mut t = vec![0; size];

    t[1] = 1;

    for i in 2..size {
        t[i] = (t[i - 1] + t[i - 2]) % modulus;
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

        let t = fibonacci(n, MOD);

        assert_eq!(t[..10].to_vec(), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34,]);
    }
}
