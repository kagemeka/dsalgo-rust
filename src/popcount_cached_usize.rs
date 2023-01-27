use std::collections::HashMap;

pub struct Popcount(HashMap<usize, usize>);

impl Popcount {
    pub fn new() -> Self { Self(HashMap::new()) }

    pub fn calc(
        &mut self,
        n: usize,
    ) -> usize {
        if let Some(&c) = self.0.get(&n) {
            c
        } else {
            let c = if n == 0 { 0 } else { self.calc(n >> 1) + (n & 1) };

            self.0.insert(n, c);

            c
        }
    }
}

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

        let mut f = Popcount::new();

        for (n, ans) in cases {
            assert_eq!(f.calc(n), ans);
        }
    }
}
