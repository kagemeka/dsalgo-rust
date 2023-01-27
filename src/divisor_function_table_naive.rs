/// O(N\log{N}\log{k})

pub fn divisor_func(
    k: usize,
    size: usize,
) -> Vec<usize> {
    let k = k as u32;

    let mut f = vec![0; size];

    for i in 1..size {
        let x = i.pow(k);

        for j in (i..size).step_by(i) {
            f[j] += x;
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
