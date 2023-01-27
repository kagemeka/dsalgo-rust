use crate::{
    algebraic_structure::*,
    power_group::pow_group,
};

pub trait PowGroup: Group
where
    Self::S: Clone,
{
    fn pow_group(
        x: Self::S,
        exp: i64,
    ) -> Self::S {
        pow_group(&Self::op, &Self::e, &Self::inv, x, exp)
    }
}

impl<T: Group> PowGroup for T where T::S: Clone {}
