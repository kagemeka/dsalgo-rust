use crate::sqrt_decomposition_lazy_with_instance_homomorphism::*;

pub struct RangeAddRangeSumI64;

impl Ops for RangeAddRangeSumI64 {
    type F = i64;

    type S = (i64, i64);

    fn op(
        &self,
        a: Self::S,
        b: Self::S,
    ) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }

    fn e(&self) -> Self::S { (0, 0) }

    fn compose(
        &self,
        f: Self::F,
        g: Self::F,
    ) -> Self::F {
        f + g
    }

    fn id(&self) -> Self::F { 0 }

    fn map(
        &self,
        f: Self::F,
        x: Self::S,
    ) -> Self::S {
        (x.0 + f * x.1, x.1)
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
