pub fn to_int<T: From<i32>>(x: bool) -> T {
    if x {
        1.into()
    } else {
        0.into()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(to_int::<i32>(true), 1);
    }
}
