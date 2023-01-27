/// compute DFT/IDFT on complex f64 precision
use std::f64::consts::*;

use crate::complex_number_f64::Complex;

fn dft_core(
    a: &[Complex],
    zeta_sign: f64,
) -> Vec<Complex> {
    let n = a.len();

    (0..n)
        .map(|i| {
            let mut b = Complex(0., 0.);

            for (j, &a) in a.iter().enumerate() {
                b += a * Complex::from_polar(
                    1.0,
                    zeta_sign * 2. * PI * i as f64 * j as f64 / n as f64,
                );
            }

            b
        })
        .collect()
}

pub fn dft(a: &[Complex]) -> Vec<Complex> { dft_core(a, -1.0) }

pub fn idft(b: &[Complex]) -> Vec<Complex> {
    let mut a = dft_core(b, 1.0);

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

            f = dft(&f);

            g = dft(&g);

            for i in 0..dft_len {
                f[i] *= g[i];
            }

            f = idft(&f);

            dbg!(&f);

            let f: Vec<_> = f.iter().map(|x| x.rint() as i64).collect();

            assert_eq!(&f[1..=n << 1], ans);
        }
    }
}
