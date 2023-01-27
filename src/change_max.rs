pub fn chmax<T: Ord>(
    a: &mut T,
    b: T,
) {
    if b > *a {
        *a = b;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a = 2;

        chmax(&mut a, 1);

        assert_eq!(a, 2);

        chmax(&mut a, 3);

        assert_eq!(a, 3);
    }
}
