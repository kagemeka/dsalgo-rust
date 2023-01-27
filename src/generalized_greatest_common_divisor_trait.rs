/// generalized greatest common divisor on commutative ring R.
/// gcd(a, b) might not be unique neither exist.
/// if R does not have Order property.

pub trait GCD {
    type T;

    fn gcd(
        _: Self::T,
        _: Self::T,
    ) -> Self::T;
}
