use std::ops::*;

pub fn mobius_divisors<T: Clone, F: Fn(T, T) -> T>(
    op: F,
    mut f: Vec<T>,
) -> Vec<T> {
    let n = f.len();

    for i in 1..n {
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

        let res = number_of_divisors(n);

        assert_eq!(mobius_divisors(|a, b| a - b, res), a);
    }
}
