/// O(N\log{N})

pub fn sum_of_divisors(size: usize) -> Vec<usize> {
    let mut cnt = vec![0; size];

    for i in 1..size {
        for j in (i..size).step_by(i) {
            cnt[j] += i;
        }
    }

    cnt
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;

        const ANS: &[usize] = &[
            1, 3, 4, 7, 6, 12, 8, 15, 13, 18, 12, 28, 14, 24, 24, 31, 18, 39,
            20, 42, 32, 36, 24, 60, 31, 42, 40, 56, 30, 72, 32, 63, 48, 54, 48,
            91, 38, 60, 56, 90, 42, 96, 44, 84, 78, 72, 48, 124, 57, 93, 72,
            98, 54, 120, 72, 120, 80, 90, 60, 168, 62, 96, 104, 127, 84, 144,
            68, 126, 96, 144,
        ];

        let n = ANS.len();

        let sigma = sum_of_divisors(n + 1);

        assert_eq!(&sigma[1..], ANS);
    }
}
