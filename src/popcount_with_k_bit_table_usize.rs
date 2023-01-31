use crate::popcount_table::popcount as table;

pub struct Popcount(Vec<usize>);

impl Popcount {
    pub fn new(k: usize) -> Self {
        Self(table(1 << k).into_iter().map(|c| c as usize).collect())
    }

    pub fn calc(
        &self,
        mut n: usize,
    ) -> usize {
        let k = self.0.len().trailing_zeros();

        let mask = (1 << k) - 1;

        let mut cnt = 0;

        while n > 0 {
            cnt += self.0[n & mask];

            n >>= k;
        }

        cnt
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [
            (0b1010, 2),
            (0b1100100, 3),
            (0b001010101, 4),
            (0b11111, 5),
            (0b00000, 0),
        ];

        let f = Popcount::new(8);

        for (n, ans) in cases {
            assert_eq!(f.calc(n), ans);
        }
    }
}
