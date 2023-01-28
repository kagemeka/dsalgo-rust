use std::ops::*;

pub fn pascal_triangle<T>(size: usize) -> Vec<Vec<T>>
where
    T: Add<Output = T> + From<i32> + Clone,
{
    let mut p: Vec<Vec<T>> = vec![vec![0.into(); size]; size];

    for i in 0..size {
        p[i][0] = 1.into();
    }

    for i in 1..size {
        for j in 1..=i {
            p[i][j] = p[i - 1][j - 1].clone() + p[i - 1][j].clone();
        }
    }

    p
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::{
            define_const_modulus_macro::Mod1_000_000_007I64,
            modular_int_with_static_modulus::Modint,
        };

        type Mint = Modint<Mod1_000_000_007I64>;

        let p = pascal_triangle::<Mint>(6);

        let mut inner = vec![vec![0; 6]; 6];

        for i in 0..6 {
            for j in 0..6 {
                inner[i][j] = p[i][j].0;
            }
        }

        assert_eq!(
            inner,
            vec![
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0],
                vec![1, 2, 1, 0, 0, 0],
                vec![1, 3, 3, 1, 0, 0],
                vec![1, 4, 6, 4, 1, 0],
                vec![1, 5, 10, 10, 5, 1],
            ]
        );
    }
}
