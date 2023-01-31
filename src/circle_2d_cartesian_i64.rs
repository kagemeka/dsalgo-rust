use crate::dist_2d_to_the_power_of_2::dist2;

/// (x, y, radius)

pub struct Circle(i64, i64, i64);

impl Circle {
    pub fn dist2(
        &self,
        rhs: &Self,
    ) -> i64 {
        (self.0 - rhs.0).pow(2) + (self.1 - rhs.1).pow(2)
    }

    pub fn intersect(
        &self,
        rhs: &Self,
    ) -> bool {
        let d = self.dist2(rhs);

        (self.2 - rhs.2).pow(2) <= d && d <= (self.2 + rhs.2).pow(2)
    }

    /// radius power to the 2.

    pub fn r2(&self) -> i64 { self.2.pow(2) }

    pub fn dist2_from_center(
        &self,
        x: i64,
        y: i64,
    ) -> i64 {
        dist2(self.0, self.1, x, y)
    }

    pub fn on_circumference(
        &self,
        x: i64,
        y: i64,
    ) -> bool {
        self.dist2_from_center(x, y) == self.r2()
    }
}

impl From<(i64, i64, i64)> for Circle {
    fn from(c: (i64, i64, i64)) -> Self { Self(c.0, c.1, c.2) }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (((0, 0, 1), (0, 0, 2)), false),
            (((0, 0, 2), (3, 3, 3)), true),
            (((2, 3, 1), (2, 0, 2)), true),
        ];

        for ((c0, c1), ans) in cases {
            assert_eq!(Circle::from(c0).intersect(&c1.into()), ans);
        }
    }
}
