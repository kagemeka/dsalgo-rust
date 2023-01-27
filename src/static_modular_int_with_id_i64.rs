use std::{
    marker::PhantomData,
    sync::atomic::{
        AtomicI64,
        Ordering::SeqCst,
    },
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Modint<Id>(pub i64, PhantomData<Id>);

impl<Id> std::fmt::Display for Modint<Id> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Id: Copy> Modint<Id> {
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

    pub fn new(v: i64) -> Self { Self(Self::normalize(v), PhantomData) }
}

use std::ops::*;

impl<Id: Copy> Add for Modint<Id> {
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

impl<Id: Copy> Neg for Modint<Id> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.0 != 0 {
            self.0 = Self::m() - self.0;
        }

        self
    }
}

impl<Id: Copy> Mul for Modint<Id> {
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

impl<Id: Copy> MulInv for Modint<Id> {
    type Output = Self;

    fn mul_inv(mut self) -> Self::Output {
        self.0 = modinv(Self::m(), self.0);

        self
    }
}

impl<Id: Copy> Sub for Modint<Id> {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self + -rhs
    }
}

impl<Id: Copy> Div for Modint<Id> {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self * rhs.mul_inv()
    }
}

impl<Id: Copy> Add<i64> for Modint<Id> {
    type Output = Self;

    fn add(
        self,
        rhs: i64,
    ) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<Id: Copy> Sub<i64> for Modint<Id> {
    type Output = Self;

    fn sub(
        self,
        rhs: i64,
    ) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<Id: Copy> Mul<i64> for Modint<Id> {
    type Output = Self;

    fn mul(
        self,
        rhs: i64,
    ) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<Id: Copy> Div<i64> for Modint<Id> {
    type Output = Self;

    fn div(
        self,
        rhs: i64,
    ) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<Id: Copy> Add<Modint<Id>> for i64 {
    type Output = Modint<Id>;

    fn add(
        self,
        rhs: Modint<Id>,
    ) -> Self::Output {
        Modint::<Id>::new(self) + rhs
    }
}

impl<Id: Copy> Sub<Modint<Id>> for i64 {
    type Output = Modint<Id>;

    fn sub(
        self,
        rhs: Modint<Id>,
    ) -> Self::Output {
        Modint::<Id>::new(self) - rhs
    }
}

impl<Id: Copy> Mul<Modint<Id>> for i64 {
    type Output = Modint<Id>;

    fn mul(
        self,
        rhs: Modint<Id>,
    ) -> Self::Output {
        Modint::<Id>::new(self) * rhs
    }
}

impl<Id: Copy> Div<Modint<Id>> for i64 {
    type Output = Modint<Id>;

    fn div(
        self,
        rhs: Modint<Id>,
    ) -> Self::Output {
        Modint::<Id>::new(self) / rhs
    }
}

impl<Id: Copy, T> AddAssign<T> for Modint<Id>
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

impl<Id: Copy, T> SubAssign<T> for Modint<Id>
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

impl<Id: Copy, T> MulAssign<T> for Modint<Id>
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

impl<Id: Copy, T> DivAssign<T> for Modint<Id>
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

impl<Id: Copy> Modint<Id> {
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

impl<Id: Copy> From<i32> for Modint<Id> {
    fn from(x: i32) -> Self { Self::new(x as i64) }
}

impl<Id: Copy> From<usize> for Modint<Id> {
    fn from(x: usize) -> Self { Self::new(x as i64) }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct DefaultId;

pub type Mint = Modint<DefaultId>;

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        #[derive(Copy, Clone, Hash, PartialEq, Eq)]

        struct Id;

        type Mint = Modint<Id>;

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
