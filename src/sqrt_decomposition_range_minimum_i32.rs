use crate::sqrt_decomposition_with_instance_monoid::*;

pub struct RangeMinimum;

impl Monoid for RangeMinimum {
    type T = i32;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T {
        l.min(r)
    }

    fn e(&self) -> Self::T { std::i32::MAX }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
