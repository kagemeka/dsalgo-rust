use std::ops::*;

/// O(3^N)

pub fn zeta_subset<T: Clone + Add<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let m = f.len();

    for s in (0..m).rev() {
        let mut t = s;

        while t > 0 {
            t = (t - 1) & s;

            f[s] = f[s].clone() + f[t].clone();
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

        f = zeta_subset(f);

        assert_eq!(
            f,
            [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3,]
        );
    }
}
