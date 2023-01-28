#[macro_export]

macro_rules! impl_add_assign_from_add {
    ($type:ty) => {
        impl<T> AddAssign<T> for $type
        where
            T: Clone,
            Self: Clone + Add<T, Output = Self>,
        {
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                *self = self.clone() + rhs.clone();
            }
        }
    };
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use std::ops::*;

        #[derive(Clone)]

        struct A(usize);

        impl Add for A {
            type Output = Self;

            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl_add_assign_from_add!(A);
    }
}
