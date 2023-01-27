use crate::sqrt_decomposition_dual_with_instance_monoid::*;

pub struct RangeUpdate;

impl Monoid for RangeUpdate {
    type T = Option<i32>;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T {
        if r.is_none() {
            l
        } else {
            r
        }
    }

    fn e(&self) -> Self::T { None }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
