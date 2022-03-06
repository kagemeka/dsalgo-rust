pub struct Additive;
pub struct Multiplicative;

pub trait Identity<S = Self, T = Additive> {
    fn identity() -> S;
}

pub trait DynamicIdentity<S = Self, T = Additive> {
    fn identity(&self) -> S;
}

pub trait BinaryOperation<S = Self, T = Additive> {
    fn operate(lhs: &S, rhs: &S) -> S;
}

pub trait Associative<S = Self, T = Additive>: BinaryOperation<S, T> {}

pub trait Idempotent<S = Self, T = Additive>: BinaryOperation<S, T> {}

pub trait Commutative<S = Self, T = Additive>: BinaryOperation<S, T> {}

pub trait Inverse<S = Self, T = Additive> {
    fn invert(element: &S) -> S;
}

pub trait Magma<S = Self, T = Additive>: BinaryOperation<S, T> {}
impl<S, T, U: BinaryOperation<S, T>> Magma<S, T> for U {}

// pub trait Semigroup<S = Self, T = Additive>:
// BinaryOperation<S, T>{} impl<S, T, U: BinaryOperation<S, T>>
// Semigroup<S, T> for U {}

pub trait Semigroup<S = Self, T = Additive>: Magma<S, T> + Associative<S, T> {}
impl<S, T, U: Magma<S, T> + Associative<S, T>> Semigroup<S, T> for U {}

pub trait Monoid<S = Self, T = Additive>: Semigroup<S, T> + Identity<S, T> {}
impl<S, T, U: Semigroup<S, T> + Identity<S, T>> Monoid<S, T> for U {}

pub trait Group<S = Self, T = Additive>: Monoid<S, T> + Inverse<S, T> {}
impl<S, T, U: Monoid<S, T> + Inverse<S, T>> Group<S, T> for U {}

pub trait AbelianGroup<S = Self, T = Additive>: Group<S, T> + Commutative<S, T> {}
impl<S, T, U: Group<S, T> + Commutative<S, T>> AbelianGroup<S, T> for U {}

pub trait Semiring<S = Self, Add = Additive, Mul = Multiplicative>:
    Monoid<S, Add> + Monoid<S, Mul> + Commutative<S, Add>
{
}
impl<S, Add, Mul, U> Semiring<S, Add, Mul> for U where
    U: Monoid<S, Add> + Monoid<S, Mul> + Commutative<S, Add>
{
}

pub trait Ring<S = Self, Add = Additive, Mul = Multiplicative>:
    Semiring<S, Add, Mul> + Inverse<S, Add>
{
}
impl<S, Add, Mul, U> Ring<S, Add, Mul> for U where U: Semiring<S, Add, Mul> + Inverse<S, Add> {}

pub trait Default<S = Self, T = Additive> {
    fn default() -> S;
}

/// example of more concrete traits
pub trait AdditiveGroup<S>: AbelianGroup<S, Additive> {}
impl<S, U: AbelianGroup<S, Additive>> AdditiveGroup<S> for U {}

#[cfg(test)]
mod tests {
    // struct ExampleSemiring<S>(std::marker::PhantomData<S>);
    struct UsizeAddMul;

    impl super::Identity<usize, super::Additive> for UsizeAddMul {
        fn identity() -> usize { 0 }
    }

    impl super::Identity<usize, super::Multiplicative> for UsizeAddMul {
        fn identity() -> usize { 1 }
    }

    impl super::BinaryOperation<usize, super::Additive> for UsizeAddMul {
        fn operate(a: &usize, b: &usize) -> usize { a + b }
    }

    impl super::Associative<usize, super::Additive> for UsizeAddMul {}

    impl super::BinaryOperation<usize, super::Multiplicative> for UsizeAddMul {
        fn operate(a: &usize, b: &usize) -> usize { a * b }
    }

    impl super::Associative<usize, super::Multiplicative> for UsizeAddMul {}

    impl super::Commutative<usize, super::Additive> for UsizeAddMul {}

    fn need_semiring<S, Add, Mul, U>()
    where
        U: super::Semiring<S, Add, Mul>,
        S: std::fmt::Debug + PartialEq,
    {
        let add_e = <U as super::Identity<S, Add>>::identity();
        let value_add = <U as super::BinaryOperation<S, Add>>::operate(&add_e, &add_e);
        assert_eq!(value_add, add_e);

        let mul_e = <U as super::Identity<S, Mul>>::identity();
        let value_mul = <U as super::BinaryOperation<S, Mul>>::operate(&mul_e, &mul_e);
        assert_eq!(value_mul, mul_e);
        eprintln!("{:?}", value_add);
    }

    #[test]
    fn test() { need_semiring::<usize, super::Additive, super::Multiplicative, UsizeAddMul>(); }
}