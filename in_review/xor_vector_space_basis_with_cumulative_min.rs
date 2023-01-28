pub fn xor_basis(a: &[u64]) -> Vec<u64> {
    let mut basis = vec![];

    for &x in a {
        let mut x = x;

        for y in basis.clone().into_iter() {
            x = x.min(x ^ y);
        }

        if x != 0 {
            basis.push(x);
        }
    }

    basis
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {}
}
