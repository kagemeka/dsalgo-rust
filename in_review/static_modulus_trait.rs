pub trait Get {
    type T;

    fn get() -> Self::T;
}

pub trait Set {
    type T;

    fn set(value: Self::T);
}
