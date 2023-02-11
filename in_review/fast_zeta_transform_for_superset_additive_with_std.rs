use std::ops::*;

/// O(N*2^N)

pub fn fast_zeta_superset<T: Clone + Add<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let m = f.len();

    let n = m.next_power_of_two().trailing_zeros();

    for i in 0..n {
        for s in 0..m {
            let t = s | 1 << i;

            if s < t && t < m {
                f[s] = f[s].clone() + f[t].clone();
            }
        }
    }

    f
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
