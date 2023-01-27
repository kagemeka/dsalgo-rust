use std::sync::atomic::{
    AtomicI64,
    Ordering::SeqCst,
};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]

pub struct Modint(pub i64);

impl Modint {
    fn cell() -> &'static AtomicI64 {
        static CELL: AtomicI64 = AtomicI64::new(0);

        &CELL
    }

    pub fn get_mod() -> i64 { Self::cell().load(SeqCst) }

    fn m() -> i64 { Self::get_mod() }

    pub fn set_mod(value: i64) { Self::cell().store(value, SeqCst); }

    pub fn normalize(mut v: i64) -> i64 {
        let m = Self::m();

        if v < -Self::m() || v >= m {
            v %= m;
        }

        if v < 0 {
            v += m;
        }

        v
    }

    pub fn new(v: i64) -> Self { Self(Self::normalize(v)) }
}

use std::ops::*;

impl Add for Modint {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self.0 += rhs.0;

        if self.0 >= Self::m() {
            self.0 -= Self::m();
        }

        self
    }
}

impl Neg for Modint {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.0 != 0 {
            self.0 = Self::m() - self.0;
        }

        self
    }
}

impl Mul for Modint {
    type Output = Self;

    fn mul(
        mut self,
        rhs: Self,
    ) -> Self::Output {
        self.0 *= rhs.0;

        if self.0 >= Self::m() {
            self.0 %= Self::m();
        }

        self
    }
}

use crate::{
    modular_inverse_euclidean_i64_no_error::modinv,
    multiplicative_inverse::MulInv,
};

impl MulInv for Modint {
    type Output = Self;

    fn mul_inv(mut self) -> Self::Output {
        self.0 = modinv(Self::m(), self.0);

        self
    }
}

impl Sub for Modint {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self + -rhs
    }
}

impl Div for Modint {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self * rhs.mul_inv()
    }
}

impl Add<i64> for Modint {
    type Output = Self;

    fn add(
        self,
        rhs: i64,
    ) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl Sub<i64> for Modint {
    type Output = Self;

    fn sub(
        self,
        rhs: i64,
    ) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl Mul<i64> for Modint {
    type Output = Self;

    fn mul(
        self,
        rhs: i64,
    ) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl Div<i64> for Modint {
    type Output = Self;

    fn div(
        self,
        rhs: i64,
    ) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl Add<Modint> for i64 {
    type Output = Modint;

    fn add(
        self,
        rhs: Modint,
    ) -> Self::Output {
        Modint::new(self) + rhs
    }
}

impl Sub<Modint> for i64 {
    type Output = Modint;

    fn sub(
        self,
        rhs: Modint,
    ) -> Self::Output {
        Modint::new(self) - rhs
    }
}

impl Mul<Modint> for i64 {
    type Output = Modint;

    fn mul(
        self,
        rhs: Modint,
    ) -> Self::Output {
        Modint::new(self) * rhs
    }
}

impl Div<Modint> for i64 {
    type Output = Modint;

    fn div(
        self,
        rhs: Modint,
    ) -> Self::Output {
        Modint::new(self) / rhs
    }
}

impl<T> AddAssign<T> for Modint
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

impl<T> SubAssign<T> for Modint
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

impl<T> MulAssign<T> for Modint
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

impl<T> DivAssign<T> for Modint
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

impl Modint {
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

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        type Mint = Modint;

        Mint::set_mod(MOD);

        let mut x = Mint::new(-1);

        assert_eq!(x.0, 1_000_000_006);

        x += 2;

        assert_eq!(x.0, 1);

        assert_eq!((5 * x).0, 5);

        x.0 = 2;

        assert_eq!(x.pow(-1).0, (MOD + 1) >> 1);
    }
}
