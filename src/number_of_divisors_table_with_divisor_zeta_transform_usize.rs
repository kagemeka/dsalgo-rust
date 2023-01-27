/// d(n) table. d(0) := 0 here.
/// O(N\log{N})

pub fn number_of_divisors(size: usize) -> Vec<usize> {
    let mut f = vec![1; size];

    f[0] = 0;

    for i in (1..size).rev() {
        for j in (i << 1..size).step_by(i) {
            f[j] += f[i];
        }
    }

    f
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;

        assert_eq!(number_of_divisors(10), [0, 1, 2, 2, 3, 2, 4, 2, 4, 3]);
    }
}
