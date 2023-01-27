/// mu(0) is undefined, please don't access to.

pub fn mobius_function(size: usize) -> Vec<isize> {
    let mut f = vec![0; size];

    f[1] = 1;

    let inf = 1 << 60;

    for i in 2..size {
        if f[i] != 0 {
            // not prime
            if f[i] < 0 {
                f[i] = 0;
            } else if f[i] & 1 == 1 {
                f[i] = -1;
            } else {
                f[i] = 1;
            }

            continue;
        }

        f[i] = -1;

        for j in (i << 1..size).step_by(i) {
            f[j] += 1;
        }

        for j in (i * i..size).step_by(i * i) {
            f[j] = -inf;
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
