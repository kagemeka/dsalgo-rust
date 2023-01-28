use crate::{
    algebraic_structure::itself::*,
    power_group::pow_group,
};

pub trait PowGroup<I>: Group<I>
where
    Self: Clone,
{
    fn pow_group(
        self,
        exp: i64,
    ) -> Self {
        pow_group(&Self::op, &Self::e, &Self::inv, self, exp)
    }
}

impl<S: Group<I> + Clone, I> PowGroup<I> for S {}
