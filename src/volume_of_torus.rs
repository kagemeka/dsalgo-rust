pub fn volume(
    x: f64,
    r: f64,
) -> f64 {
    2.0 * (std::f64::consts::PI * r).powf(2.0) * x
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        dbg!(volume(10.0, 2.0));
    }
}
