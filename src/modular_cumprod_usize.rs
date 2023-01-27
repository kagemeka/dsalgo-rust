pub fn cumprod(
    m: usize,
    mut a: Vec<usize>,
) -> Vec<usize> {
    for i in 0..a.len() - 1 {
        a[i + 1] *= a[i];

        a[i + 1] %= m;
    }

    a
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
