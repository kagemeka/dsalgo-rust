pub trait ModularArithmetic {
    type T;

    fn modulus(&self) -> Self::T;

    fn add(
        &self,
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T;

    fn neg(
        &self,
        x: Self::T,
    ) -> Self::T;

    fn sub(
        &self,
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T {
        self.add(lhs, self.neg(rhs))
    }

    fn mul(
        &self,
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T;

    fn inv(
        &self,
        x: Self::T,
    ) -> Self::T;

    fn div(
        &self,
        lhs: Self::T,
        rhs: Self::T,
    ) -> Self::T {
        self.mul(lhs, self.inv(rhs))
    }
}
