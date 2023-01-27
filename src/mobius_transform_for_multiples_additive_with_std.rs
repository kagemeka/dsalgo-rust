use std::ops::*;

pub fn mobius_multiples<T: Clone + Sub<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let n = f.len();

    for i in (1..n).rev() {
        for j in (i << 1..n).step_by(i) {
            f[i] = f[i].clone() - f[j].clone();
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::number_of_multiples_table_naive_usize::number_of_multiples;

        let n = 1 << 15;

        let mut a = vec![1; n];

        a[0] = 0;

        let res = number_of_multiples(n);

        assert_eq!(mobius_multiples(res), a);
    }
}
