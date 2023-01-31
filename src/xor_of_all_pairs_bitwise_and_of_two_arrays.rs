use std::{
    iter::Sum,
    ops::*,
};

pub fn xor_of_all_bitwise_and<T>(
    a0: &[T],
    a1: &[T],
    max_bit: usize,
) -> T
where
    T: Clone
        + Shl<usize, Output = T>
        + Shr<usize, Output = T>
        + Mul<Output = T>
        + AddAssign
        + BitXor<Output = T>
        + BitAnd<Output = T>
        + From<i32>
        + Sum,
{
    let mut s = 0.into();

    let one: T = 1.into();

    for i in 0..max_bit {
        let c0 = a0.iter().map(|x| x.clone() >> i & 1.into()).sum::<T>();

        let c1 = a1.iter().map(|x| x.clone() >> i & 1.into()).sum::<T>();

        s += (one.clone() << i) * (c0 * c1 & 1.into());
    }

    s
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases =
            vec![((vec![1, 2, 3], vec![5, 6]), 0), ((vec![12], vec![4]), 4)];

        for ((a0, a1), ans) in cases {
            assert_eq!(xor_of_all_bitwise_and(&a0, &a1, 30), ans)
        }
    }
}
