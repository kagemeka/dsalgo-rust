pub fn factorial<T>(n: u64) -> T
where
    T: std::ops::Mul<Output = T> + From<u64>,
{
    (1..=n).fold(1.into(), |accum, x| accum * x.into())
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;
        use crate::{
            default_static_modular_arithmetic::Modular1_000_000_007,
            modular_int_with_arithmetic::Modint,
        };

        type Mint = Modint<u32, Modular1_000_000_007>;

        assert_eq!(factorial::<Mint>(20).value(), 146326063);
    }
}
