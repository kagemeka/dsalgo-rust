use std::ops::*;

use crate::sqrt_decomposition_lazy_with_instance_homomorphism::*;

pub struct RangeAddRangeMinimum<T>(T, T);

impl<T> RangeAddRangeMinimum<T> {
    pub fn new(
        inf: T,
        zero: T,
    ) -> Self {
        Self(inf, zero)
    }
}

impl<T> Ops for RangeAddRangeMinimum<T>
where
    T: Ord + Clone + Add<Output = T>,
{
    type F = T;

    type S = T;

    fn op(
        &self,
        a: Self::S,
        b: Self::S,
    ) -> Self::S {
        a.min(b)
    }

    fn e(&self) -> Self::S { self.0.clone() }

    fn compose(
        &self,
        f: Self::F,
        g: Self::F,
    ) -> Self::F {
        f + g
    }

    fn id(&self) -> Self::F { self.1.clone() }

    fn map(
        &self,
        f: Self::F,
        x: Self::S,
    ) -> Self::S {
        if x == self.e() {
            x
        } else {
            f + x
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
