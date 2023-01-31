/// use \sum_{d|n} mu(n) = 1 if n = 1 else 0

pub fn mobius_function(size: usize) -> Vec<isize> {
    let mut f = vec![0; size];

    f[1] = 1;

    for i in 1..size {
        for j in (i << 1..size).step_by(i) {
            f[j] -= f[i];
        }
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MU: &[isize] = &[
            1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0,
            1, 1, -1, 0, 0, 1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0,
            -1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1, 0, -1, 0, 1, 0, 1, 1, -1, 0,
            -1, 1, 0, 0, 1, -1, -1, 0, 1, -1, -1, 0, -1, 1, 0, 0, 1, -1,
        ];

        let n = MU.len();

        let mu = mobius_function(n + 1);

        assert_eq!(&mu[1..], MU);
    }
}
