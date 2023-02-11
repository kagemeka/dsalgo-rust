/// mu(0) is undefined, please don't access to.

pub fn mobius_function(size: usize) -> Vec<isize> {
    let mut f = vec![0; size];

    f[1] = 1;

    for i in 2..size {
        // fill mu(i)
        if f[i] != 0 {
            let lpf = f[i] as usize;

            let j = i / lpf;

            f[i] = if j % lpf == 0 { 0 } else { f[j] * -1 };

            continue;
        }

        f[i] = -1;

        // update lpf for i|j
        for j in (i * i..size).step_by(i) {
            if f[j] == 0 {
                f[j] = i as isize;
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
