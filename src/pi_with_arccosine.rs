pub fn pi() -> f64 { (-1f64).acos() }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(pi(), std::f64::consts::PI);
    }
}
