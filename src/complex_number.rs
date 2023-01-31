use std::ops::*;

use crate::number_trait::*;

/// (real, imaginary)

pub struct Complex<T>(pub T, pub T);

impl<T: Clone + Number> Complex<T> {
    pub fn i() -> Self { Self(T::zero(), T::one()) }

    pub fn norm_square(&self) -> T {
        self.0.clone() * self.0.clone() + self.1.clone() * self.1.clone()
    }

    pub fn scale(
        &self,
        t: T,
    ) -> Self {
        Self(self.0.clone() * t.clone(), self.1.clone() * t)
    }

    pub fn unscale(
        &self,
        t: T,
    ) -> Self {
        Self(self.0.clone() / t.clone(), self.1.clone() / t)
    }
}

impl<T: Clone + Number + Neg<Output = T>> Complex<T> {
    pub fn conjugate(&self) -> Self { Self(self.0.clone(), -self.1.clone()) }

    pub fn inverse(&self) -> Self {
        let ns = self.norm_square();

        Self(self.0.clone() / ns.clone(), -self.1.clone() / ns)
    }
}

// TODO:
