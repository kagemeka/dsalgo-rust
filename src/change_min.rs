pub fn chmin<T: Ord>(
    a: &mut T,
    b: T,
) {
    if b < *a {
        *a = b;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a = 2;

        chmin(&mut a, 3);

        assert_eq!(a, 2);

        chmin(&mut a, 1);

        assert_eq!(a, 1);
    }
}
