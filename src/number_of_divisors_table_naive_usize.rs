/// d(n) table. d(0) := 0 here.
/// O(N\log{N})

pub fn number_of_divisors(size: usize) -> Vec<usize> {
    let mut cnt = vec![0; size];

    for i in 1..size {
        for j in (i..size).step_by(i) {
            cnt[j] += 1;
        }
    }

    cnt
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;

        assert_eq!(number_of_divisors(10), [0, 1, 2, 2, 3, 2, 4, 2, 4, 3]);
    }
}
