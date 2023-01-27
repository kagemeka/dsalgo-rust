/// O(N)

pub fn sum_of_multiples(size: usize) -> Vec<usize> {
    (0..size)
        .map(|x| {
            if x == 0 {
                0
            } else {
                let c = (size - 1) / x;

                (1 + c) * c / 2 * x
            }
        })
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let f = sum_of_multiples(10);

        assert_eq!(f, [0, 45, 20, 18, 12, 5, 6, 7, 8, 9]);
    }
}
