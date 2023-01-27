pub fn cumprod(
    m: i64,
    mut a: Vec<i64>,
) -> Vec<i64> {
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
