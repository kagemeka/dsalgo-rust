use std::ops::*;

use crate::{accumulate_iterator::accumulate, factorial::factorial};

pub fn inverse_factorial_table<T>(size: usize) -> Vec<T>
where
    T: Mul<Output = T> + Div<Output = T> + From<u64> + Clone,
{
    if size == 0 {
        return vec![];
    }

    let mut v = (0..size as u64).map(|i| (i + 1).into()).collect::<Vec<T>>();

    if size == 0 {
        return v;
    }

    v[size - 1] = T::from(1) / factorial::<T>(size as u64 - 1);

    let op = |a: T, b: T| -> T { a * b };

    let mut ifact = accumulate(&op, v.into_iter().rev()).collect::<Vec<_>>();

    ifact.reverse();

    ifact
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

        let res = inverse_factorial_table::<Mint>(20)
            .into_iter()
            .map(|x| x.value())
            .collect::<Vec<u32>>();

        assert_eq!(
            res,
            [
                1, 1, 500000004, 166666668, 41666667, 808333339, 301388891,
                900198419, 487524805, 831947206, 283194722, 571199524,
                380933296, 490841026, 320774361, 821384963, 738836565,
                514049213, 639669405, 402087866
            ],
        );
    }
}
