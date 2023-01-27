/// able to compute (k <= 2^max_exp)-th from any node in O(max_exp) time

pub fn doubling_table(
    f: &[usize],
    n_bits: usize,
) -> Vec<Vec<usize>> {
    assert!(n_bits > 0);

    let n = f.len();

    let mut a = Vec::with_capacity(n_bits);

    a.push(f.to_owned());

    for i in 0..n_bits - 1 {
        let row = &a[i];

        a.push((0..n).map(|j| row[row[j]]).collect());
    }

    a
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
