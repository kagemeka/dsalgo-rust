use crate::{algebraic_structure::itself::*, power_monoid::pow_monoid};

pub trait PowMonoid<I>: Monoid<I>
where
    Self: Clone,
{
    fn pow_monoid(self, exp: u64) -> Self {
        pow_monoid(&Self::op, &Self::e, self, exp)
    }
}

impl<S: Monoid<I> + Clone, I> PowMonoid<I> for S {}
