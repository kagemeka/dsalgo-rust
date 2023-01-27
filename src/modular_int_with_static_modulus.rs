use std::ops::*;

pub trait Element:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Rem<Output = Self>
    + RemAssign
    + Copy
    + Ord
    + Eq
    + From<i32>
    + Into<i64>
{
}

impl<T> Element for T where
    T: Sized
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + DivAssign
        + Neg<Output = Self>
        + Rem<Output = Self>
        + RemAssign
        + Copy
        + Ord
        + Eq
        + From<i32>
        + Into<i64>
{
}

use crate::static_modulus_trait::Get;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Modint<M: Get>(pub M::T);

impl<M: Get> Modint<M>
where
    M::T: Element,
{
    pub fn modulus() -> M::T { M::get() }

    fn m() -> M::T { M::get() }

    pub fn normalize(mut v: M::T) -> M::T {
        let m = Self::m();

        if v < -Self::m() || v >= m {
            v %= m;
        }

        if v < 0.into() {
            v += m;
        }

        v
    }

    pub fn new(v: M::T) -> Self { Self(Self::normalize(v)) }
}

impl<M: Get> Add for Modint<M>
where
    M::T: Element,
{
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

impl<M: Get> Neg for Modint<M>
where
    M::T: Element,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.0 != 0.into() {
            self.0 = Self::m() - self.0;
        }

        self
    }
}

impl<M: Get> Mul for Modint<M>
where
    M::T: Element,
{
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

impl<M: Get> MulInv for Modint<M>
where
    M::T: Element,
{
    type Output = Self;

    fn mul_inv(mut self) -> Self::Output {
        self.0 = (modinv(Self::m().into(), self.0.into()) as i32).into();

        self
    }
}

impl<M: Get> Sub for Modint<M>
where
    M::T: Element,
{
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self + -rhs
    }
}

impl<M: Get> Div for Modint<M>
where
    M::T: Element,
{
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self * rhs.mul_inv()
    }
}

impl<M: Get + Copy, T> AddAssign<T> for Modint<M>
where
    Self: Add<T, Output = Self>,
    M::T: Element,
{
    fn add_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self + rhs;
    }
}

impl<M: Get + Copy, T> SubAssign<T> for Modint<M>
where
    Self: Sub<T, Output = Self>,
    M::T: Element,
{
    fn sub_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self - rhs;
    }
}

impl<M: Get + Copy, T> MulAssign<T> for Modint<M>
where
    Self: Mul<T, Output = Self>,
    M::T: Element,
{
    fn mul_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self * rhs;
    }
}

impl<M: Get + Copy, T> DivAssign<T> for Modint<M>
where
    Self: Div<T, Output = Self>,
    M::T: Element,
{
    fn div_assign(
        &mut self,
        rhs: T,
    ) {
        *self = *self / rhs;
    }
}

impl<M: Get + Copy> Modint<M>
where
    M::T: Element,
{
    pub fn pow(
        self,
        n: i64,
    ) -> Self {
        if n < 0 {
            return self.mul_inv().pow(-n);
        }

        if n == 0 {
            return Self::new(1.into());
        }

        let mut y = self.pow(n >> 1);

        y *= y;

        if n & 1 == 1 {
            y *= self;
        }

        y
    }
}

impl<M: Get + Copy> From<i32> for Modint<M>
where
    M::T: Element,
{
    fn from(v: i32) -> Self { Self::new(v.into()) }
}

impl<M: Get + Copy> From<usize> for Modint<M>
where
    M::T: Element,
{
    fn from(v: usize) -> Self { (v as i32).into() }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::define_const_modulus_macro::Mod1_000_000_007I64;

        type Mint = Modint<Mod1_000_000_007I64>;

        let mut x = Mint::new(-1);

        assert_eq!(x.0, 1_000_000_006);

        x += Mint::new(2);

        assert_eq!(x.0, 1);

        assert_eq!((Mint::new(5) * x).0, 5);

        x.0 = 2;

        assert_eq!(x.pow(-1).0, (Mint::modulus() + 1) >> 1);
    }
}
