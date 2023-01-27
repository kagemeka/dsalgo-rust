pub mod modulus {

    pub trait StaticGet {
        fn get() -> u32;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

    pub struct Mod1_000_000_007;

    impl StaticGet for Mod1_000_000_007 {
        fn get() -> u32 { 1_000_000_007 }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

    pub struct Mod998_244_353;

    impl StaticGet for Mod998_244_353 {
        fn get() -> u32 { 998_244_353 }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

    pub struct StaticMod;

    use std::sync::atomic::{
        AtomicU32,
        Ordering::SeqCst,
    };

    impl StaticMod {
        fn cell() -> &'static AtomicU32 {
            static CELL: AtomicU32 = AtomicU32::new(0);

            &CELL
        }

        pub fn set(value: u32) { Self::cell().store(value, SeqCst); }
    }

    impl StaticGet for StaticMod {
        fn get() -> u32 { Self::cell().load(SeqCst) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

pub struct Modint<M>(pub u32, std::marker::PhantomData<M>);

use std::ops::*;

use modulus::StaticGet;

use crate::multiplicative_inverse::MulInv;

impl<M: StaticGet> Modint<M> {
    pub fn modulus() -> u32 { M::get() }

    pub fn new(mut v: u32) -> Self {
        if v >= M::get() {
            v %= M::get();
        }

        Self(v, std::marker::PhantomData)
    }
}

impl<M: StaticGet> Add for Modint<M> {
    type Output = Self;

    fn add(
        mut self,
        rhs: Self,
    ) -> Self {
        self.0 += rhs.0;

        if self.0 >= M::get() {
            self.0 -= M::get()
        }

        self
    }
}

impl<M: StaticGet> Neg for Modint<M> {
    type Output = Self;

    fn neg(mut self) -> Self {
        if self.0 != 0 {
            self.0 = M::get() - self.0
        }

        self
    }
}

impl<M: StaticGet> Mul for Modint<M> {
    type Output = Self;

    fn mul(
        mut self,
        rhs: Self,
    ) -> Self {
        let mut v = self.0 as u64;

        v *= rhs.0 as u64;

        let m = M::get() as u64;

        if v >= m {
            v %= m;
        }

        self.0 = v as u32;

        self
    }
}

impl<M: StaticGet> MulInv for Modint<M> {
    type Output = Self;

    fn mul_inv(mut self) -> Self {
        use std::mem::swap;

        let (mut a, mut b) = (self.0 as i64, M::get() as i64);

        let (mut x00, mut x01) = (1, 0);

        while b != 0 {
            x00 -= a / b * x01;

            a %= b;

            swap(&mut a, &mut b);

            swap(&mut x00, &mut x01);
        }

        assert_eq!(a, 1);

        if x00 < 0 {
            x00 += M::get() as i64;
        }

        debug_assert!(0 <= x00 && x00 < M::get() as i64);

        self.0 = x00 as u32;

        self
    }
}

impl<M: StaticGet> Sub for Modint<M> {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self {
        self + -rhs
    }
}

impl<M: StaticGet> Div for Modint<M> {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self {
        self * rhs.mul_inv()
    }
}

impl<M: StaticGet> AddAssign for Modint<M>
where
    Self: Copy,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self + rhs;
    }
}

impl<M: StaticGet> SubAssign for Modint<M>
where
    Self: Copy,
{
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        *self += -rhs;
    }
}

impl<M: StaticGet> MulAssign for Modint<M>
where
    Self: Copy,
{
    fn mul_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self * rhs;
    }
}

impl<M: StaticGet> DivAssign for Modint<M>
where
    Self: Copy,
{
    fn div_assign(
        &mut self,
        rhs: Self,
    ) {
        *self *= *self * rhs.mul_inv();
    }
}

impl<M: StaticGet> From<i64> for Modint<M> {
    fn from(mut v: i64) -> Self {
        let m = M::get() as i64;

        if v < -m || m <= v {
            v %= m;
        }

        if v < 0 {
            v += m;
        }

        Self::new(v as u32)
    }
}

impl<M: StaticGet> From<u64> for Modint<M> {
    fn from(mut v: u64) -> Self {
        let m = M::get() as u64;

        if v >= m {
            v %= m;
        }

        Self::new(v as u32)
    }
}
