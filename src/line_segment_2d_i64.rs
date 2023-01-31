use crate::geometric_vector_2d_i64::*;

pub struct Segment(Vector2D, Vector2D);

impl From<((i64, i64), (i64, i64))> for Segment {
    fn from(points: ((i64, i64), (i64, i64))) -> Self {
        let (p0, p1) = points;

        Self(p0.into(), p1.into())
    }
}

impl Segment {
    pub fn across(
        &self,
        other: &Self,
    ) -> bool {
        let v0 = self.1 - self.0;

        let v1 = other.0 - self.0;

        let v2 = other.1 - self.0;

        v0.cross(&v1).signum() * v0.cross(&v2).signum() <= 0
    }

    pub fn intersect(
        &self,
        other: &Self,
    ) -> bool {
        self.across(other) && other.across(self)
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
