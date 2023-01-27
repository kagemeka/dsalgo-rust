use std::ops::*;

use crate::{
    const_generics_modular_int_i64::Modint,
    segment_tree_lazy_additive_homomorphism_with_std_ops::*,
};

type Mint = Modint<998_244_353>;

#[derive(Clone)]

pub struct S(Mint, Mint);

impl Add for S {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        S(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Identity for S {
    fn e() -> S { S(0.into(), 0.into()) }
}

#[derive(Clone)]

pub struct F(Option<Mint>);

impl Add for F {
    type Output = Self;

    fn add(
        self,
        g: Self,
    ) -> Self::Output {
        if g.0.is_some() {
            g
        } else {
            self
        }
    }
}

impl Identity for F {
    fn e() -> Self { F(None) }
}

impl Add<F> for S {
    type Output = Self;

    fn add(
        self,
        f: F,
    ) -> Self::Output {
        if let Some(f) = f.0 {
            S(f * self.1.clone(), self.1)
        } else {
            self
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
