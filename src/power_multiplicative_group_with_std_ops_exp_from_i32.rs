use std::ops::*;

pub fn power<T, Z>(
    mut x: T,
    mut n: Z,
) -> T
where
    T: Clone + MulAssign + Div<Output = T> + From<i32>,
    Z: From<i32>
        + Ord
        + SubAssign
        + Neg<Output = Z>
        + ShrAssign<usize>
        + BitAnd<Output = Z>
        + Copy,
{
    let zero = 0.into();

    let one = 1.into();

    if n < zero {
        x = T::from(1) / x;

        n = -n;
    }

    let mut y = 1.into();

    while n > zero {
        if n & one == one {
            y *= x.clone();
        }

        x *= x.clone();

        n >>= 1;
    }

    y
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::{
            default_static_modular_arithmetic::Modular1_000_000_007,
            modular_int_with_arithmetic::Modint,
        };

        type Mint = Modint<u32, Modular1_000_000_007>;

        let a: Mint = 2.into();

        assert_eq!(power(a, 10).value(), 1024);

        assert_eq!(power(a, 0).value(), 1);

        assert_eq!(power(a, -1).value(), (1_000_000_008) >> 1);
    }
}
