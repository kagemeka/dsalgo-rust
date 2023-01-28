pub fn xor_basis_original(a: &[usize]) -> Vec<usize> {
    let m = *a.iter().max().unwrap() + 1;

    let mut ok = vec![false; m];

    ok[0] = true;

    let mut res = Vec::new();

    for &x in a.iter() {
        if ok[x] {
            continue;
        }

        res.push(x);

        for i in 0..m {
            ok[i ^ x] |= ok[i];
        }
    }

    res
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
