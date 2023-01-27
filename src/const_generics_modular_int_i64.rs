#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Modint<const MOD: i64>(pub i64);

impl<const MOD: i64> Modint<MOD> {
    pub const fn modulus() -> i64 { MOD }

    pub fn normalize(mut v: i64) -> i64 {
        if v < -MOD || v >= MOD {
            v %= MOD;
        }

        if v < 0 {
            v += MOD;
        }

        v
    }

    pub fn new(v: i64) -> Self { Self(Self::normalize(v)) }
}

use std::ops::*;

impl<const MOD: i64> Add for Modint<MOD> {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self.0 += rhs.0;

        if self.0 >= MOD {
            self.0 -= MOD;
        }

        self
    }
}

impl<const MOD: i64> Neg for Modint<MOD> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.0 != 0 {
            self.0 = MOD - self.0;
        }

        self
    }
}

impl<const MOD: i64> Mul for Modint<MOD> {
    type Output = Self;

    fn mul(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self.0 *= rhs.0;

        if self.0 >= MOD {
            self.0 %= MOD;
        }

        self
    }
}

use crate::{
    modular_inverse_euclidean_i64_no_error::modinv,
    multiplicative_inverse::MulInv,
};

impl<const MOD: i64> MulInv for Modint<MOD> {
    type Output = Self;

    fn mul_inv(mut self) -> Self::Output {
        self.0 = modinv(MOD, self.0);

        self
    }
}

impl<const MOD: i64> Sub for Modint<MOD> {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self + -rhs
    }
}

impl<const MOD: i64> Div for Modint<MOD> {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self * rhs.mul_inv()
    }
}

impl<const MOD: i64> Add<i64> for Modint<MOD> {
    type Output = Self;

    fn add(
        self,
        rhs: i64,
    ) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MOD: i64> Sub<i64> for Modint<MOD> {
    type Output = Self;

    fn sub(
        self,
        rhs: i64,
    ) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MOD: i64> Mul<i64> for Modint<MOD> {
    type Output = Self;

    fn mul(
        self,
        rhs: i64,
    ) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MOD: i64> Div<i64> for Modint<MOD> {
    type Output = Self;

    fn div(
        self,
        rhs: i64,
    ) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MOD: i64> Add<Modint<MOD>> for i64 {
    type Output = Modint<MOD>;

    fn add(
        self,
        rhs: Modint<MOD>,
    ) -> Self::Output {
        Modint::<MOD>::new(self) + rhs
    }
}

impl<const MOD: i64> Sub<Modint<MOD>> for i64 {
    type Output = Modint<MOD>;

    fn sub(
        self,
        rhs: Modint<MOD>,
    ) -> Self::Output {
        Modint::<MOD>::new(self) - rhs
    }
}

impl<const MOD: i64> Mul<Modint<MOD>> for i64 {
    type Output = Modint<MOD>;

    fn mul(
        self,
        rhs: Modint<MOD>,
    ) -> Self::Output {
        Modint::<MOD>::new(self) * rhs
    }
}

impl<const MOD: i64> Div<Modint<MOD>> for i64 {
    type Output = Modint<MOD>;

    fn div(
        self,
        rhs: Modint<MOD>,
    ) -> Self::Output {
        Modint::<MOD>::new(self) / rhs
    }
}

impl<const MOD: i64, T> AddAssign<T> for Modint<MOD>
where
    Self: Add<T, Output = Self>,
{
    fn add_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64, T> SubAssign<T> for Modint<MOD>
where
    Self: Sub<T, Output = Self>,
{
    fn sub_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64, T> MulAssign<T> for Modint<MOD>
where
    Self: Mul<T, Output = Self>,
{
    fn mul_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64, T> DivAssign<T> for Modint<MOD>
where
    Self: Div<T, Output = Self>,
{
    fn div_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self / rhs;
    }
}

impl<const MOD: i64> Modint<MOD> {
    pub fn pow(
        self,
        n: i64,
    ) -> Self {
        if n < 0 {
            return self.mul_inv().pow(-n);
        }

        if n == 0 {
            return Self::new(1);
        }

        let mut y = self.pow(n >> 1);

        y *= y;

        if n & 1 == 1 {
            y *= self;
        }

        y
    }
}

impl<const MOD: i64> From<i32> for Modint<MOD> {
    fn from(x: i32) -> Self { Self::new(x as i64) }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        type Mint = Modint<MOD>;

        let mut x = Mint::new(-1);

        assert_eq!(x.0, 1_000_000_006);

        x += 2;

        assert_eq!(x.0, 1);

        assert_eq!((5 * x).0, 5);

        x.0 = 2;

        assert_eq!(x.pow(-1).0, (MOD + 1) >> 1);
    }
}
