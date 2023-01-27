use crate::sqrt_decomposition_lazy_with_instance_homomorphism::*;

pub struct RangeUpdateRangeMinimum<T>(T);

impl<T> RangeUpdateRangeMinimum<T> {
    pub fn new(inf: T) -> Self { Self(inf) }
}

impl<T: Ord + Clone> Ops for RangeUpdateRangeMinimum<T> {
    type F = Option<T>;

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
            f
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
