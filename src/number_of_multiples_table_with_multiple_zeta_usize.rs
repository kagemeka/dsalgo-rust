/// O(N\log{N})

pub fn number_of_multiples(size: usize) -> Vec<usize> {
    let mut f = vec![1; size];

    f[0] = 0;

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
        let f = number_of_multiples(10);

        assert_eq!(f, [0, 9, 4, 3, 2, 1, 1, 1, 1, 1]);
    }
}
