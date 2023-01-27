use crate::{
    algebraic_structure::*,
    power_monoid::*,
};

pub trait PowMonoid: Monoid
where
    Self::S: Clone,
{
    fn pow_monoid(
        x: Self::S,
        exp: u64,
    ) -> Self::S {
        pow_monoid(&Self::op, &Self::e, x, exp)
    }
}

impl<T: Monoid> PowMonoid for T where T::S: Clone {}
