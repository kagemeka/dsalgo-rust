/// O(N\log{N})

pub fn sum_of_multiples(size: usize) -> Vec<usize> {
    let mut f: Vec<_> = (0..size).collect();

    for i in 1..size {
        for j in (i << 1..size).step_by(i) {
            f[i] += f[j];
        }
    }

    f
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
