use crate::binary_function::*;

pub trait Magma: BinaryOp {}

impl<T: BinaryOp> Magma for T {}

pub trait Semigroup: Magma + Associative {}

impl<T: Magma + Associative> Semigroup for T {}

// TODO:
// pub trait InvSemigroup
pub trait Monoid: Semigroup + Identity {}

impl<T> Monoid for T where T: Semigroup + Identity {}

pub trait UnitalMagma: Magma + Identity {}

impl<T: Magma + Identity> UnitalMagma for T {}

pub trait Quasigroup: Magma + LatinSquare {}

impl<T: Magma + LatinSquare> Quasigroup for T {}

pub trait Loop: Quasigroup + Identity {}

impl<T: Quasigroup + Identity> Loop for T {}

pub trait Group: Monoid + Inverse {}

impl<T: Monoid + Inverse> Group for T {}

pub trait AbelianGroup: Group + Commutative {}

impl<T: Group + Commutative> AbelianGroup for T {}

pub trait Semiring: Zero + One + Distributive {}

impl<T: Zero + One + Distributive> Semiring for T {}

pub trait Ring: Semiring + AddInv {}

impl<T: Semiring + AddInv> Ring for T {}

pub mod dynamic {

    use crate::binary_function::dynamic::*;

    pub trait Magma: BinaryOp {}

    impl<T: BinaryOp> Magma for T {}
}

pub mod itself {

    use crate::binary_function::itself::*;

    pub trait Magma<I: Id>: BinaryOp<I> {}

    impl<I, T> Magma<I> for T
    where
        T: BinaryOp<I>,
        I: Id,
    {
    }

    pub trait Semigroup<I: Id>: Magma<I> + Associative<I> {}

    impl<I, T> Semigroup<I> for T
    where
        T: Magma<I> + Associative<I>,
        I: Id,
    {
    }

    pub trait Monoid<I: Id>: Semigroup<I> + Identity<I> {}

    impl<I, T> Monoid<I> for T
    where
        T: Semigroup<I> + Identity<I>,
        I: Id,
    {
    }

    pub trait Group<I: Id>: Monoid<I> + Inverse<I> {}

    impl<I, T> Group<I> for T
    where
        T: Monoid<I> + Inverse<I>,
        I: Id,
    {
    }
}
