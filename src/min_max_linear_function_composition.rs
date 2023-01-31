//! deal with 3 kinds of composition queries.
//! f(x) := f(x) + a
//! f(x) := max(f(x), a)
//! f(x) := min(f(x), a)

const INF: i64 = std::i64::MAX;

pub struct MinMaxLinear {
    delta: i64,
    low: i64,
    high: i64,
}

impl MinMaxLinear {
    pub fn new() -> Self { Self { delta: 0, low: -INF, high: INF } }

    pub fn add(
        &mut self,
        v: i64,
    ) {
        self.delta += v;

        if self.low != -INF {
            self.low += v;
        }

        if self.high != INF {
            self.high += v;
        }
    }

    pub fn max(
        &mut self,
        v: i64,
    ) {
        self.low = self.low.max(v);

        self.high = self.high.max(v);
    }

    pub fn min(
        &mut self,
        v: i64,
    ) {
        self.low = self.low.min(v);

        self.high = self.high.min(v);
    }

    pub fn calc(
        &self,
        x: i64,
    ) -> i64 {
        (x + self.delta).min(self.high).max(self.low)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            vec![(-10, 2), (10, 1), (10, 3)],
            vec![(-15, 0), (-10, 0), (-5, 5), (0, 10), (5, 10)],
        )];

        for (at, q) in cases {
            let mut f = MinMaxLinear::new();

            for &(a, t) in at.iter() {
                if t == 1 {
                    f.add(a);
                } else if t == 2 {
                    f.max(a);
                } else if t == 3 {
                    f.min(a);
                }
            }

            for &(x, y) in q.iter() {
                assert_eq!(f.calc(x), y);
            }
        }
    }
}
