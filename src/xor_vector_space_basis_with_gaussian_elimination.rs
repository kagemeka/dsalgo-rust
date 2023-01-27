pub fn xor_basis(
    a: &[usize],
    n_bits: usize,
) -> Vec<usize> {
    let mut basis = vec![0; n_bits];

    for &x in a.iter() {
        let mut x = x;

        for (i, b) in basis.iter().enumerate() {
            if x >> i & 1 == 1 {
                x ^= b;
            }
        }

        if x == 0 {
            continue;
        }

        for (i, b) in basis.iter_mut().enumerate() {
            if x >> i & 1 == 0 {
                continue;
            }

            assert!(*b == 0);

            *b = x;

            for (j, b) in basis.iter_mut().enumerate() {
                if i == j {
                    continue;
                }

                if *b >> i & 1 == 1 {
                    *b ^= x;
                }
            }

            break;
        }
    }

    basis.into_iter().filter(|b| *b != 0).collect()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
