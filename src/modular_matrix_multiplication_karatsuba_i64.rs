use crate::{
    matrix_addition_i64::add,
    matrix_multiplication_i64::mul,
    matrix_subtraction_i64::sub,
};

pub fn mod_mul(
    modulus: i64,
    a: &[Vec<i64>],
    b: &[Vec<i64>],
) -> Vec<Vec<i64>> {
    const B: u8 = 16;

    const M: i64 = (1 << B) - 1;

    assert!(modulus < 1 << (B << 1));

    let split = |a: &[Vec<i64>]| -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
        // a = a1 * B^1 + a0 * B^0
        let h = a.len();

        let w = a[0].len();

        let mut a0 = vec![vec![0; w]; h];

        let mut a1 = vec![vec![0; w]; h];

        for (i, row) in a.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                a0[i][j] = x & M;

                a1[i][j] = x >> B;
            }
        }

        (a0, a1)
    };

    let merge =
        |a0: &[Vec<i64>], a1: &[Vec<i64>], a2: &[Vec<i64>]| -> Vec<Vec<i64>> {
            let h = a0.len();

            let w = a0[0].len();

            let mut a = vec![vec![0; w]; h];

            for i in 0..h {
                for j in 0..w {
                    let x = &mut a[i][j];

                    *x += a2[i][j] << (B << 1);

                    *x += a1[i][j] << B;

                    *x += a0[i][j];
                }
            }

            a
        };

    let take_mod = |a: &mut [Vec<i64>]| {
        for row in a.iter_mut() {
            for x in row.iter_mut() {
                *x %= modulus;

                if *x < 0 {
                    *x += modulus;
                }
            }
        }
    };

    let (a0, a1) = split(a);

    let (b0, b1) = split(b);

    let mut c0 = mul(&a0, &b0);

    take_mod(&mut c0);

    let mut c2 = mul(&a1, &b1);

    take_mod(&mut c2);

    let mut c1 = mul(&add(&a0, &a1), &add(&b0, &b1));

    c1 = sub(&sub(&c1, &c0), &c2);

    take_mod(&mut c1);

    let mut c = merge(&c0, &c1, &c2);

    take_mod(&mut c);

    c
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![vec![0, 1], vec![2, 3], vec![4, 5]];

        let b = vec![vec![0, 1, 2], vec![3, 4, 5]];

        let c = mod_mul(1_000_000_007, &a, &b);

        assert_eq!(c, [[3, 4, 5,], [9, 14, 19,], [15, 24, 33,]]);
    }
}
