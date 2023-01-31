use crate::sqrt_decomposition_dual_with_instance_monoid::*;

pub struct RangeAdd;

impl Monoid for RangeAdd {
    type T = i64;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T {
        l + r
    }

    fn e(&self) -> Self::T { 0 }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
