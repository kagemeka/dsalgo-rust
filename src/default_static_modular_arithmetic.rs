use crate::{
    modular_inverse_euclidean_u64::modinv,
    static_modular_arithmetic_trait::ModularArithmetic,
    static_modulus_trait::Get,
};

/// why `default`?
/// because there exists other modular arithmetic implementations.
/// e.g. Montgomery Multiplication, or Burrett Reduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]

pub struct DefaultStatic<T, M: Get<T = T>>(std::marker::PhantomData<(T, M)>);

macro_rules! impl_default_static {
    ($uint:ty, $mul_cast_uint:ty) => {
        impl<M: Get<T = $uint>> ModularArithmetic for DefaultStatic<$uint, M> {
            type T = $uint;

            fn modulus() -> Self::T { M::get() }

            fn add(
                lhs: Self::T,
                rhs: Self::T,
            ) -> Self::T {
                assert!(lhs < M::get() && rhs < M::get());

                let mut x = lhs;

                x += rhs;

                if x >= M::get() {
                    x -= M::get();
                }

                x
            }

            fn neg(x: Self::T) -> Self::T {
                assert!(x < M::get());

                if x == 0 {
                    0
                } else {
                    M::get() - x
                }
            }

            fn mul(
                lhs: Self::T,
                rhs: Self::T,
            ) -> Self::T {
                let mut x = lhs as $mul_cast_uint;

                x *= rhs as $mul_cast_uint;

                x %= M::get() as $mul_cast_uint;

                x as Self::T
            }

            fn inv(x: $uint) -> Self::T {
                assert!(x > 0);

                modinv(M::get() as u64, x as u64).unwrap() as Self::T
            }
        }
    };
}

impl_default_static!(u32, u64);

impl_default_static!(u64, u128);

// TODO: change later. still not compile on AtCoder.
// use crate::modular::modulus::ConstMod32;
// #[allow(dead_code)]
// pub type Modular1_000_000_007 =
//     DefaultStatic<u32, ConstMod32<1_000_000_007>>;
// #[allow(dead_code)]
// pub type Modular998_244_353 =
//     DefaultStatic<u32, ConstMod32<998_244_353>>;
use crate::define_const_modulus_macro::{
    Mod1_000_000_007,
    Mod998_244_353,
};

#[allow(dead_code)]

pub type Modular1_000_000_007 = DefaultStatic<u32, Mod1_000_000_007>;

#[allow(dead_code)]

pub type Modular998_244_353 = DefaultStatic<u32, Mod998_244_353>;

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::modular_int_with_arithmetic::Modint;

        type Mint = Modint<u32, Modular1_000_000_007>;

        let a = Mint::from(1_000_000_008);

        assert_eq!(a.value(), 1);
    }
}
