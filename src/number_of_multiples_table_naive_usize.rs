/// O(N)

pub fn number_of_multiples(size: usize) -> Vec<usize> {
    (0..size).map(|x| if x == 0 { 0 } else { (size - 1) / x }).collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let f = number_of_multiples(10);

        assert_eq!(f, [0, 9, 4, 3, 2, 1, 1, 1, 1, 1]);
    }
}
