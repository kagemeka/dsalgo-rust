use std::ops::*;

pub fn sum_xor_for_all_subsets<T>(
    modulus: T,
    a: &[T],
) -> T
where
    T: Copy
        + Mul<Output = T>
        + BitXor<Output = T>
        + From<i32>
        + BitOr<Output = T>
        + MulAssign
        + RemAssign
        + Rem<Output = T>,
{
    let mut p = 1.into();

    for _ in 0..a.len() - 1 {
        p *= 2.into();

        p %= modulus;
    }

    a.to_owned().into_iter().reduce(|a, b| a | b).unwrap() * p % modulus
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(vec![1, 3], 6), (vec![1, 5, 6], 28)];

        const MOD: i32 = 1_000_000_007;

        for (a, ans) in cases {
            assert_eq!(sum_xor_for_all_subsets(MOD, &a), ans);
        }
    }
}
