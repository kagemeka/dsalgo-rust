pub fn xor_basis_original(
    a: &[usize],
    n_bits: usize,
) -> Vec<usize> {
    let mut basis = vec![0; n_bits];

    let mut res = Vec::with_capacity(n_bits);

    for &v in a.iter() {
        let mut x = v;

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

            res.push(v);

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

    res
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
