use crate::greatest_common_divisor_euclidean::gcd;

pub fn is_setwise_coprime(a: &[usize]) -> bool {
    a.to_vec().into_iter().reduce(gcd).unwrap() == 1
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert!(is_setwise_coprime(&[6, 10, 15]));
    }
}
