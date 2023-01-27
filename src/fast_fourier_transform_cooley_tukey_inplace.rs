//! Fast Fourier Transform
//! Cooley-Tukey's Algorithm
//! Iterative Inplace

use std::f64::consts::*;

use crate::complex_number_f64::*;

fn fft_core(
    mut a: Vec<Complex>,
    bit_len: usize,
    zeta_sign: f64,
) -> Vec<Complex> {
    use std::mem::size_of;

    let n = 1 << bit_len;

    a.resize(n, Complex::zero());

    // bit reversal permutate (butterfly)
    for i in 0..n {
        let j = i.reverse_bits() >> (size_of::<usize>() << 3) - bit_len;

        if i < j {
            a.swap(i, j);
        }
    }

    // divide and conquer along with inverse butterfly
    let mut d = 1;

    while d < n {
        for j in 0..d {
            let w =
                Complex::from_polar(1.0, zeta_sign * PI / d as f64 * j as f64);

            for i in (j..n).step_by(d << 1) {
                let s = a[i];

                let t = a[i + d] * w;

                a[i] = s + t;

                a[i + d] = s - t;
            }
        }

        d <<= 1;
    }

    a
}

pub fn fft(
    a: Vec<Complex>,
    bit_len: usize,
) -> Vec<Complex> {
    fft_core(a, bit_len, -1.0)
}

pub fn ifft(
    mut a: Vec<Complex>,
    bit_len: usize,
) -> Vec<Complex> {
    a = fft_core(a, bit_len, 1.0);

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
