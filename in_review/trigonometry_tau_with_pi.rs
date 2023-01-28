pub fn tau() -> f64 { std::f64::consts::PI * 2. }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(tau(), std::f64::consts::TAU);
    }
}
