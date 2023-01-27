use crate::{
    algebraic_structure::*,
    power_semigroup::pow_semigroup,
};

pub trait PowSemigroup: Semigroup
where
    Self::S: Clone,
{
    fn pow_seimigroup(
        x: Self::S,
        exp: u64,
    ) -> Self::S {
        pow_semigroup(&Self::op, x, exp)
    }
}

impl<T: Semigroup> PowSemigroup for T where T::S: Clone {}
