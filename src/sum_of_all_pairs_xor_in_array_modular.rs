use std::{
    iter::Sum,
    ops::*,
};

pub fn sum_of_xor<T>(
    modulus: T,
    a: &[T],
    max_bit: usize,
) -> T
where
    T: Copy
        + Shl<usize, Output = T>
        + Shr<usize, Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign
        + BitXor<Output = T>
        + BitAnd<Output = T>
        + From<i32>
        + Sum
        + Rem<Output = T>,
{
    let n: T = (a.len() as i32).into();

    let mut s: T = 0.into();

    let one: T = 1.into();

    for i in 0..max_bit {
        let c = a.iter().map(|&x| x >> i & 1.into()).sum::<T>();

        s += (one << i) % modulus * c % modulus * (n - c) % modulus;
    }

    s % modulus
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        let cases = vec![(
            vec![
                3i64, 14, 159, 2653, 58979, 323846, 2643383, 27950288,
                419716939, 9375105820,
            ],
            103715602,
        )];

        for (a, ans) in cases {
            assert_eq!(sum_of_xor(MOD, &a, 60), ans);
        }
    }
}
