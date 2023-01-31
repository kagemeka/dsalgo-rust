//! Fast Fourier Transform
//! Cooley-Tukey's Algorithm
//! Recurse

use std::f64::consts::*;

use crate::complex_number_f64::*;

fn fft_core(
    mut a: Vec<Complex>,
    zeta_sign: f64,
) -> Vec<Complex> {
    let n = a.len();

    assert_eq!(n.count_ones(), 1);

    if n == 1 {
        return a;
    }

    let a0 = fft_core(a[..].iter().step_by(2).map(|x| *x).collect(), zeta_sign);

    let a1 =
        fft_core(a[1..].iter().step_by(2).map(|x| *x).collect(), zeta_sign);

    for (i, (s, mut t)) in a0.into_iter().zip(a1.into_iter()).enumerate() {
        t *= Complex::from_polar(
            1.0,
            zeta_sign * 2.0 * PI / n as f64 * i as f64,
        );

        a[i] = s + t;

        a[i + (n >> 1)] = s - t;
    }

    a
}

pub fn fft(
    mut a: Vec<Complex>,
    bit_len: usize,
) -> Vec<Complex> {
    a.resize(1 << bit_len, Complex::zero());

    fft_core(a, -1.0)
}

pub fn ifft(
    mut a: Vec<Complex>,
    bit_len: usize,
) -> Vec<Complex> {
    a.resize(1 << bit_len, Complex::zero());

    a = fft_core(a, 1.0);

    let n = a.len() as f64;

    for x in a.iter_mut() {
        *x /= n;
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atc001_fft_c_convolution() {
        let cases = vec![(
            vec![(1, 1), (2, 2), (3, 4), (4, 8)],
            vec![0, 1, 4, 11, 26, 36, 40, 32],
        )];

        for (ab, ans) in cases {
            let n = ab.len();

            let dft_len = (n << 1) + 1;

            let mut f = Vec::with_capacity(dft_len);

            let mut g = Vec::with_capacity(dft_len);

            f.push(Complex(0.0, 0.0));

            g.push(Complex(0.0, 0.0));

            for (a, b) in ab {
                f.push(Complex(a as f64, 0.0));

                g.push(Complex(b as f64, 0.0));
            }

            f.resize(dft_len, Complex(0.0, 0.0));

            g.resize(dft_len, Complex(0.0, 0.0));

            let bit_len = dft_len.next_power_of_two().trailing_zeros() as usize;

            f = fft(f, bit_len);

            g = fft(g, bit_len);

            for i in 0..1 << bit_len {
                f[i] *= g[i];
            }

            f = ifft(f, bit_len);

            dbg!(&f);

            let f: Vec<_> = f.iter().map(|x| x.rint() as i64).collect();

            assert_eq!(&f[1..=n << 1], ans);
        }
    }
}
