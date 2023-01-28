use crate::matrix_with_static_property::*;

#[derive(Debug, Clone)]

pub struct R(usize);

impl From<i32> for R {
    fn from(x: i32) -> Self {
        assert!(x == 0 || x == 1);

        Self(x as usize * std::usize::MAX)
    }
}

use std::ops::*;

impl Add for R {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self.0 ^= rhs.0;

        self
    }
}

impl AddAssign for R {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone() + rhs;
    }
}

impl Mul for R {
    type Output = Self;

    fn mul(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self.0 &= rhs.0;

        self
    }
}

pub type BitXorAndMat<P> = Matrix<R, P>;

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
