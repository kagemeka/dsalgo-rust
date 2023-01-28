use std::ops::*;

pub fn fast_zeta_divisors<T, F>(
    mut f: Vec<T>,
    op: F,
) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> T,
{
    let n = f.len();

    for i in (1..n).rev() {
        for j in (i << 1..n).step_by(i) {
            f[j] = op(f[j].clone(), f[i].clone());
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::number_of_divisors_table_naive_usize::number_of_divisors;

        let n = 1 << 15;

        let mut a = vec![1; n];

        a[0] = 0;

        let f = |a, b| a + b;

        let res = fast_zeta_divisors(a, f);

        assert_eq!(res, number_of_divisors(n));
    }
}
