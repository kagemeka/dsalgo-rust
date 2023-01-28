use std::ops::*;

/// polar form of Complex number.
/// some ops are defined in addition to basic polar system.
/// z = r(cos(theta) + i*sin(theta)) = r*exp(i*theta)
/// (r. theta)
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Polar(pub f64, pub f64);

impl Polar {
    pub fn i() -> Self { Self(1.0, std::f64::consts::PI / 2.0) }

    pub fn rectangular(&self) -> (f64, f64) {
        (self.0 * self.1.cos(), self.0 * self.1.sin())
    }

    pub fn from_rect(
        x: f64,
        y: f64,
    ) -> Self {
        Self(x.hypot(y), y.atan2(x))
    }

    pub fn mul_inv(&self) -> Self { Self(1.0 / self.0, -self.1) }
}

impl From<f64> for Polar {
    fn from(real: f64) -> Self { Self(real, 0.) }
}

impl MulAssign<f64> for Polar {
    fn mul_assign(
        &mut self,
        x: f64,
    ) {
        self.0 *= x;
    }
}

impl Mul<f64> for Polar {
    type Output = Self;

    fn mul(
        mut self,
        x: f64,
    ) -> Self::Output {
        self *= x;

        self
    }
}

impl DivAssign<f64> for Polar {
    fn div_assign(
        &mut self,
        x: f64,
    ) {
        self.0 /= x;
    }
}

impl Div<f64> for Polar {
    type Output = Self;

    fn div(
        mut self,
        x: f64,
    ) -> Self::Output {
        self /= x;

        self
    }
}

impl MulAssign for Polar {
    fn mul_assign(
        &mut self,
        rhs: Self,
    ) {
        self.0 *= rhs.0;

        self.1 += rhs.1;
    }
}

impl Mul for Polar {
    type Output = Self;

    fn mul(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self *= rhs;

        self
    }
}

impl Neg for Polar {
    type Output = Self;

    fn neg(self) -> Self::Output { Self(self.0, self.1 + std::f64::consts::PI) }
}

impl DivAssign for Polar {
    fn div_assign(
        &mut self,
        rhs: Self,
    ) {
        *self *= rhs.mul_inv();
    }
}

impl Div for Polar {
    type Output = Self;

    fn div(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self /= rhs;

        self
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
