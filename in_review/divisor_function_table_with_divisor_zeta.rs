/// O(N\log{N}\log{k})

pub fn divisor_func(
    k: usize,
    size: usize,
) -> Vec<usize> {
    let k = k as u32;

    let mut f: Vec<_> =
        (0..size).map(|i| if i == 0 { 0 } else { i.pow(k) }).collect();

    for i in (1..size).rev() {
        for j in (i << 1..size).step_by(i) {
            f[j] += f[i];
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::{
            number_of_divisors_table_naive_usize::number_of_divisors,
            sum_of_divisors_table_naive::sum_of_divisors,
        };

        let n = 20;

        let s0 = divisor_func(0, n);

        assert_eq!(s0, number_of_divisors(n));

        let s1 = divisor_func(1, n);

        assert_eq!(s1, sum_of_divisors(n));
    }
}
