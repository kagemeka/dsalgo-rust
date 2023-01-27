pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Add: std::ops::Add<Output = Self>
where
    Self: Sized,
{
}

pub trait Magma: Add {}

pub trait Semigroup: Add {}

// implicit associative property
pub trait Monoid: Add + Zero {}

// implicit associative
impl<T: Semigroup> Magma for T {}

impl<T: Monoid> Semigroup for T {}
