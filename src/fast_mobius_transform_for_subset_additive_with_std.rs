use std::ops::*;

/// O(N2^N)

pub fn fast_mobius_subset<T: Clone + Sub<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let m = f.len();

    let n = m.next_power_of_two().trailing_zeros();

    for i in 0..n {
        for s in 0..m {
            // it's not necessarily be iterated as (0..m).rev()
            // because for each bit, there are at most 2 states set/!set
            // if the other bits are same.
            let t = s | 1 << i;

            if s < t && t < m {
                f[t] = f[t].clone() - f[s].clone();
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

        let g = vec![
            0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3,
        ];

        let mut f = vec![0; n2 + 6];

        for i in 0..n + 1 {
            f[1 << i] = 1;
        }

        assert_eq!(fast_mobius_subset(g), f);
    }
}
