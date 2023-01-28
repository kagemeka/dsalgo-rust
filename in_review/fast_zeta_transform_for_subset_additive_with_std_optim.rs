use std::ops::*;

use crate::enumerate_subset_bits::enumerate_subsets;

/// O(N*2^(N - 1))

pub fn fast_zeta_subset<T: Clone + Add<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let m = f.len();

    let n = m.next_power_of_two().trailing_zeros();

    let full = (1 << n) - 1;

    for i in 0..n {
        for s in enumerate_subsets(full ^ (1 << i)) {
            let t = s | 1 << i;

            if t < m {
                f[t] = f[t].clone() + f[s].clone();
            }
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 4;

        let n2 = 1 << n;

        let mut f = vec![0; n2 + 6];

        for i in 0..n + 1 {
            f[1 << i] = 1;
        }

        f = fast_zeta_subset(f);

        assert_eq!(
            f,
            [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3,]
        );
    }
}
