use std::ops::*;

pub fn zeta_multiples<T, F>(
    op: F,
    mut f: Vec<T>,
) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> T,
{
    let n = f.len();

    for i in 1..n {
        for j in (i << 1..n).step_by(i) {
            f[i] = op(f[i].clone(), f[j].clone());
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::number_of_multiples_table_with_multiple_zeta_usize::*;

        let n = 1 << 15;

        let mut a = vec![1; n];

        a[0] = 0;

        let f = |a, b| a + b;

        let res = zeta_multiples(f, a);

        assert_eq!(res, number_of_multiples(n));
    }
}
