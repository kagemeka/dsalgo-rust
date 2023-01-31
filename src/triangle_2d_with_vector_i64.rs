use crate::geometric_vector_2d_i64::*;

pub struct Triangle(Vector2D, Vector2D, Vector2D);

impl From<((i64, i64), (i64, i64), (i64, i64))> for Triangle {
    fn from(points: ((i64, i64), (i64, i64), (i64, i64))) -> Self {
        let (p0, p1, p2) = points;

        Self(p0.into(), p1.into(), p2.into())
    }
}

impl Triangle {
    pub fn singed_area(&self) -> f64 {
        (self.1 - self.0).cross(&(self.2 - self.0)) as f64 / 2.
    }

    pub fn area(&self) -> f64 { self.singed_area().abs() }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_area() {
        let cases = vec![
            (((1, 0), (3, 0), (2, 5)), 5.0),
            (((-1, -2), (3, 4), (5, 6)), 2.0),
            (((298, 520), (903, 520), (4, 663)), 43257.5),
        ];

        for (p, ans) in cases {
            let t = Triangle::from(p);

            assert_eq!(t.area(), ans);
        }
    }
}
