use std::ops::*;

/// O(4^N)

pub fn mobius_subset<T: Clone + Sub<Output = T>>(mut f: Vec<T>) -> Vec<T> {
    let m = f.len();

    for s in 0..m {
        for t in 0..s {
            if t & s == t {
                f[s] = f[s].clone() - f[t].clone();
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

        assert_eq!(mobius_subset(g), f);
    }
}
