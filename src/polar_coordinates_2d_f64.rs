/// (r. theta)
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Polar(pub f64, pub f64);

impl Polar {
    pub fn from_cartesian(
        x: f64,
        y: f64,
    ) -> Self {
        Self(x.hypot(y), y.atan2(x))
    }

    pub fn cartesian(&self) -> (f64, f64) {
        (self.0 * self.1.cos(), self.0 * self.1.sin())
    }
}
