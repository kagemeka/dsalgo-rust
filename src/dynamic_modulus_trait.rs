pub trait Get {
    type T;

    fn get(&self) -> Self::T;
}

pub trait Set {
    type T;

    fn set(
        &mut self,
        value: Self::T,
    );
}
