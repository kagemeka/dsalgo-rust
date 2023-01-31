use std::ops::*;

use crate::{
    fast_mobius_transform_for_divisors_additive_with_std::*,
    fast_zeta_transform_for_divisors_additive_with_std::*,
};

pub fn lcm_convolve<T>(
    mut f: Vec<T>,
    mut g: Vec<T>,
) -> Vec<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    let n = f.len();

    assert_eq!(g.len(), n);

    f = fast_zeta_divisors(f);

    g = fast_zeta_divisors(g);

    for i in 0..n {
        f[i] = f[i].clone() * g[i].clone();
    }

    fast_mobius_divisors(f)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let f = vec![0, 1, 1, 1, 1];

        let res = lcm_convolve(f.clone(), f);

        assert_eq!(res, vec![0, 1, 3, 3, 5,]);
    }
}
