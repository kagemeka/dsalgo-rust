use std::ops::*;

pub fn power_of_k<T>(
    modulus: T,
    k: T,
    size: usize,
) -> Vec<T>
where
    T: Copy + Mul<Output = T> + From<u32> + Rem<Output = T>,
{
    let mut a: Vec<T> = vec![1.into(); size];

    for i in 0..size - 1 {
        a[i + 1] = k * a[i] % modulus;
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const MOD: i64 = 1_000_000_007;

        let pow_8 = power_of_k(MOD, 8, 20);

        assert_eq!(
            pow_8,
            [
                1, 8, 64, 512, 4096, 32768, 262144, 2097152, 16777216,
                134217728, 73741817, 589934536, 719476260, 755810045, 46480318,
                371842544, 974740338, 797922655, 383381198, 67049563
            ]
        );
    }
}
