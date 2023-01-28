use std::{
    marker::PhantomData,
    ops::*,
};

use crate::{
    modular_inverse_euclidean_i64_no_error::modinv,
    multiplicative_inverse::MulInv,
    static_modulus_trait::Get,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Modint<M>(pub i64, PhantomData<M>);

impl<M> std::fmt::Display for Modint<M> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M: Get<T = i64>> Modint<M> {
    pub fn modulus() -> i64 { M::get() }

    fn m() -> i64 { M::get() }

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

    pub fn new(v: i64) -> Self { Self(Self::normalize(v), PhantomData) }
}

impl<M: Get<T = i64>> Add for Modint<M> {
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

impl<M: Get<T = i64>> Neg for Modint<M> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.0 != 0 {
            self.0 = Self::m() - self.0;
        }

        self
    }
}

impl<M: Get<T = i64>> Mul for Modint<M> {
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

impl<M: Get<T = i64>> MulInv for Modint<M> {
    type Output = Self;

    fn mul_inv(mut self) -> Self::Output {
        self.0 = modinv(Self::m(), self.0);

        self
    }
}

impl<M: Get<T = i64>> Sub for Modint<M> {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self + -rhs
    }
}

impl<M: Get<T = i64>> Div for Modint<M> {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self * rhs.mul_inv()
    }
}

impl<M: Get<T = i64>> Add<i64> for Modint<M> {
    type Output = Self;

    fn add(
        self,
        rhs: i64,
    ) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<M: Get<T = i64>> Sub<i64> for Modint<M> {
    type Output = Self;

    fn sub(
        self,
        rhs: i64,
    ) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<M: Get<T = i64>> Mul<i64> for Modint<M> {
    type Output = Self;

    fn mul(
        self,
        rhs: i64,
    ) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<M: Get<T = i64>> Div<i64> for Modint<M> {
    type Output = Self;

    fn div(
        self,
        rhs: i64,
    ) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<M: Get<T = i64>> Add<Modint<M>> for i64 {
    type Output = Modint<M>;

    fn add(
        self,
        rhs: Modint<M>,
    ) -> Self::Output {
        Modint::<M>::new(self) + rhs
    }
}

impl<M: Get<T = i64>> Sub<Modint<M>> for i64 {
    type Output = Modint<M>;

    fn sub(
        self,
        rhs: Modint<M>,
    ) -> Self::Output {
        Modint::<M>::new(self) - rhs
    }
}

impl<M: Get<T = i64>> Mul<Modint<M>> for i64 {
    type Output = Modint<M>;

    fn mul(
        self,
        rhs: Modint<M>,
    ) -> Self::Output {
        Modint::<M>::new(self) * rhs
    }
}

impl<M: Get<T = i64>> Div<Modint<M>> for i64 {
    type Output = Modint<M>;

    fn div(
        self,
        rhs: Modint<M>,
    ) -> Self::Output {
        Modint::<M>::new(self) / rhs
    }
}

impl<M: Get<T = i64> + Copy, T> AddAssign<T> for Modint<M>
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

impl<M: Get<T = i64> + Copy, T> SubAssign<T> for Modint<M>
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

impl<M: Get<T = i64> + Copy, T> MulAssign<T> for Modint<M>
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

impl<M: Get<T = i64> + Copy, T> DivAssign<T> for Modint<M>
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

impl<M: Get<T = i64> + Copy> Modint<M> {
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

impl<M: Get<T = i64>> From<i32> for Modint<M> {
    fn from(x: i32) -> Self { Self::new(x as i64) }
}

impl<M: Get<T = i64>> From<usize> for Modint<M> {
    fn from(x: usize) -> Self { Self::new(x as i64) }
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

        x += 2;

        assert_eq!(x.0, 1);

        assert_eq!((5 * x).0, 5);

        x.0 = 2;

        assert_eq!(x.pow(-1).0, (Mint::modulus() + 1) >> 1);
    }
}
