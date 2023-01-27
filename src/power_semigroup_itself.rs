use crate::{
    algebraic_structure::itself::*,
    power_semigroup::pow_semigroup,
};

pub trait PowSemigroup<I>: Semigroup<I>
where
    Self: Clone,
{
    fn pow_seimigroup(
        self,
        exp: u64,
    ) -> Self {
        pow_semigroup(&Self::op, self, exp)
    }
}

impl<S: Semigroup<I> + Clone, I> PowSemigroup<I> for S {}
