pub trait CountLeadingZeros {
    type Output;

    fn clz(&self) -> Self::Output;
}
