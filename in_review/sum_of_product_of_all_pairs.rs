use std::ops::*;

pub fn sum_of_product<T>(a: &[T]) -> T
where
    T: Copy
        + Mul<Output = T>
        + AddAssign
        + Sub<Output = T>
        + From<i32>
        + Div<Output = T>,
{
    let mut s: T = 0.into();

    let mut t: T = 0.into();

    for &x in a.iter() {
        s += x;

        t += x * x;
    }

    (s * s - t) / 2.into()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::const_generics_modular_int_i64::Modint;

        type Mint = Modint<1_000_000_007>;

        let cases = vec![
            (vec![1, 2, 3], 11),
            (vec![141421356, 17320508, 22360679, 244949], 437235829),
        ];

        for (a, ans) in cases {
            let a: Vec<Mint> = a.into_iter().map(|x| x.into()).collect();

            assert_eq!(sum_of_product(&a).0, ans);
        }
    }
}
