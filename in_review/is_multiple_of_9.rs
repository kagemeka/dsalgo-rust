use crate::digits_sum::digits_sum;

pub fn is_multiple_of_9(n: u64) -> bool { digits_sum(n) % 9 == 0 }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert!(is_multiple_of_9(123456789));
    }
}
