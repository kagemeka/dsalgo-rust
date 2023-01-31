use std::ops::*;

use crate::sqrt_decomposition_lazy_with_instance_homomorphism::*;

pub struct RangeUpdateRangeSum<T>(T);

impl<T> RangeUpdateRangeSum<T> {
    pub fn new(zero: T) -> Self { Self(zero) }
}

impl<T> Ops for RangeUpdateRangeSum<T>
where
    T: Ord + Clone + Add<Output = T> + Mul<Output = T>,
{
    type F = Option<T>;

    type S = (T, T);

    fn op(
        &self,
        a: Self::S,
        b: Self::S,
    ) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }

    fn e(&self) -> Self::S { (self.0.clone(), self.0.clone()) }

    fn compose(
        &self,
        f: Self::F,
        g: Self::F,
    ) -> Self::F {
        if f.is_some() {
            f
        } else {
            g
        }
    }

    fn id(&self) -> Self::F { None }

    fn map(
        &self,
        f: Self::F,
        x: Self::S,
    ) -> Self::S {
        if let Some(f) = f {
            (f * x.1.clone(), x.1)
        } else {
            x
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
