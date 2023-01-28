//! reference
//! https://en.wikipedia.org/wiki/Modular_arithmetic#Properties

pub trait ModularArithmetic {
    type T;

    fn modulus() -> Self::T;

    fn add(
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T;

    fn neg(x: Self::T) -> Self::T;

    fn sub(
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T {
        Self::add(lhs, Self::neg(rhs))
    }

    fn mul(
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T;

    fn inv(x: Self::T) -> Self::T;

    fn div(
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T {
        Self::mul(lhs, Self::inv(rhs))
    }
}
