use std::ops::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord)]

pub struct Vector2D(i64, i64);

impl From<(i64, i64)> for Vector2D {
    fn from(p: (i64, i64)) -> Self { Self(p.0, p.1) }
}

impl AddAssign for Vector2D {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.0 += rhs.0;

        self.1 += rhs.1;
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self += rhs;

        self
    }
}

impl Neg for Vector2D {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.0 *= -1;

        self.1 *= -1;

        self
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        *self += -rhs;
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self -= rhs;

        self
    }
}

impl Mul for Vector2D {
    type Output = i64;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl MulAssign<i64> for Vector2D {
    fn mul_assign(
        &mut self,
        x: i64,
    ) {
        self.0 *= x;

        self.1 *= x;
    }
}

impl Mul<i64> for Vector2D {
    type Output = Self;

    fn mul(
        mut self,
        x: i64,
    ) -> Self::Output {
        self *= x;

        self
    }
}

impl DivAssign<i64> for Vector2D {
    fn div_assign(
        &mut self,
        x: i64,
    ) {
        assert!(self.0 % x == 0 && self.1 % x == 0);

        self.0 /= x;

        self.1 /= x;
    }
}

impl Div<i64> for Vector2D {
    type Output = Self;

    fn div(
        mut self,
        rhs: i64,
    ) -> Self::Output {
        self /= rhs;

        self
    }
}

impl Vector2D {
    pub fn cross(
        &self,
        rhs: &Self,
    ) -> i64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    /// self as origin.

    pub fn cross2(
        &self,
        a: &Self,
        b: &Self,
    ) -> i64 {
        (*a - *self).cross(&(*b - *self))
    }

    pub fn dot(
        &self,
        rhs: &Self,
    ) -> i64 {
        *self * *rhs
    }

    pub fn norm2(&self) -> i64 { self.dot(self) }

    pub fn norm(&self) -> f64 { (self.norm2() as f64).sqrt() }
}

impl Vector2D {
    /// [0, \pi): true, [-\pi, 0): false

    fn has_positive_angle(&self) -> bool {
        self.1 > 0 || self.1 == 0 && self.0 > 0
    }

    /// self < rhs ?
    /// [-\pi, \pi)

    pub fn angle_lt(
        &self,
        rhs: &Self,
    ) -> bool {
        let f = self.has_positive_angle();

        let g = rhs.has_positive_angle();

        if f != g {
            g
        } else {
            self.cross(rhs) > 0
        }
    }

    pub fn radian(&self) -> f64 { (self.1 as f64).atan2(self.0 as f64) }
}

impl Vector2D {
    pub fn is_acute(&self) -> bool { self.1 > 0 && self.0 > 0 }

    pub fn is_othogonal(&self) -> bool { self.1 > 0 && self.0 == 0 }

    pub fn is_obtuse(&self) -> bool { self.1 > 0 && self.0 < 0 }

    pub fn acute(
        &self,
        other: &Self,
    ) -> bool {
        self.cross(other) > 0 && self.dot(other) > 0
    }

    pub fn othogonal(
        &self,
        other: &Self,
    ) -> bool {
        self.cross(other) > 0 && self.dot(other) == 0
    }

    pub fn obtuse(
        &self,
        other: &Self,
    ) -> bool {
        self.cross(other) > 0 && self.dot(other) < 0
    }
}

/// compare by angle

impl PartialOrd for Vector2D {
    fn lt(
        &self,
        other: &Self,
    ) -> bool {
        self.angle_lt(other)
    }

    fn le(
        &self,
        other: &Self,
    ) -> bool {
        self == other && self < other
    }

    fn ge(
        &self,
        other: &Self,
    ) -> bool {
        !(self < other)
    }

    fn gt(
        &self,
        other: &Self,
    ) -> bool {
        !(self <= other)
    }

    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        let res = if self == other {
            Equal
        } else if self < other {
            Less
        } else {
            Greater
        };

        Some(res)
    }
}

/// rotate counter clockwise by 90k degree. (k = 1, 2, 3)

impl Vector2D {
    pub fn rot90(&self) -> Self { Self(-self.1, self.0) }

    pub fn rot180(&self) -> Self { Self(-self.0, -self.1) }

    pub fn rot270(&self) -> Self { Self(self.1, -self.0) }
}

pub enum DirectionType {
    CCW,
    CW,
    SAME,
    OPPOSITE,
}

impl Vector2D {
    /// approximated direction from self

    pub fn direction_type_of(
        &self,
        x: &Self,
    ) -> DirectionType {
        assert!(x != self);

        use DirectionType::*;

        let c = self.cross(x);

        let d = self.dot(x);

        if c > 0 {
            CCW
        } else if c < 0 {
            CW
        } else if d > 0 {
            SAME
        } else {
            OPPOSITE
        }
    }

    pub fn direction_type_from(
        &self,
        base: &Self,
    ) -> DirectionType {
        base.direction_type_of(self)
    }
}

/// ref: https://en.wikipedia.org/wiki/Angle#Types_of_angles

pub enum AngleType {
    ZERO,
    ACUTE,
    OTHOGONAL,
    OBTUSE,
    STRAIGHT,
    REFLEX,
}

pub enum TriangleAngleType {
    ACUTE,
    OTHOGONAL,
    OBTUSE,
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a = Vector2D(2, 4);

        a /= 2;

        dbg!(a);

        dbg!(a.norm());

        dbg!(a.radian());

        assert!(!a.angle_lt(&(2, 3).into()));
    }
}
