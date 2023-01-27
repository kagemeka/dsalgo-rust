/// divisors[0] := []

pub fn divisors_table(size: usize) -> Vec<Vec<usize>> {
    let mut divisors = vec![vec![]; size];

    for i in 1..size {
        for j in (i..size).step_by(i) {
            divisors[j].push(i);
        }
    }

    divisors
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let divs = divisors_table(10);

        assert_eq!(
            divs,
            [
                vec![],
                vec![1,],
                vec![1, 2,],
                vec![1, 3,],
                vec![1, 2, 4,],
                vec![1, 5,],
                vec![1, 2, 3, 6,],
                vec![1, 7,],
                vec![1, 2, 4, 8,],
                vec![1, 3, 9,],
            ]
        );
    }
}
