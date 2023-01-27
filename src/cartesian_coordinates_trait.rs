pub trait Cartesian {
    type T;

    fn x(&self) -> Self::T;

    fn y(&self) -> Self::T;
}
