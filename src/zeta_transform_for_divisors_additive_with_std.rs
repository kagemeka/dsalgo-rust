use std::ops::*;

pub fn zeta_divisors<T: Clone + Add<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let n = f.len();

    for i in (1..n).rev() {
        for j in (i << 1..n).step_by(i) {
            f[j] = f[j].clone() + f[i].clone();
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

        let res = zeta_divisors(a);

        assert_eq!(res, number_of_divisors(n));
    }
}
