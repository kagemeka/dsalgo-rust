use crate::{
    complex_number_f64::Complex,
    discrete_fourier_transform_f64::*,
};

pub fn dft_convolve(
    mut a: Vec<Complex>,
    mut b: Vec<Complex>,
) -> Vec<Complex> {
    let k = a.len() + b.len() - 1;

    a.resize(k, Complex::zero());

    b.resize(k, Complex::zero());

    let c: Vec<_> = dft(&a)
        .into_iter()
        .zip(dft(&b).into_iter())
        .map(|(a, b)| a * b)
        .collect();

    idft(&c)
}

pub fn from_i64(
    a: &[i64],
    b: &[i64],
) -> Vec<i64> {
    let a: Vec<_> = a.iter().map(|x| Complex(*x as f64, 0.0)).collect();

    let b: Vec<_> = b.iter().map(|x| Complex(*x as f64, 0.0)).collect();

    dft_convolve(a, b).into_iter().map(|x| x.rint() as i64).collect()
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

            let mut f = vec![0; n + 1];

            let mut g = vec![0; n + 1];

            for (i, &(a, b)) in ab.iter().enumerate() {
                f[i + 1] = a;

                g[i + 1] = b;
            }

            let h = from_i64(&f, &g);

            assert_eq!(&h[1..=n << 1], ans);
        }
    }
}
