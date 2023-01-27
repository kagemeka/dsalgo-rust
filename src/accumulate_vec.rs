pub fn accumulate<T, F>(
    f: F,
    mut a: Vec<T>,
) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> T,
{
    for i in 0..a.len() - 1 {
        a[i + 1] = f(a[i].clone(), a[i + 1].clone());
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a: Vec<_> = (0..10).collect();

        a = accumulate(|a, b| a + b, a);

        assert_eq!(a, [0, 1, 3, 6, 10, 15, 21, 28, 36, 45]);
    }
}
