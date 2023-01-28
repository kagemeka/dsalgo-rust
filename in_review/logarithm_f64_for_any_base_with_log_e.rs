pub fn log(
    base: f64,
    x: f64,
) -> f64 {
    const E: f64 = std::f64::consts::E;

    x.log(E) / base.log(E)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        for b in 2u64..1000 {
            let b = b as f64;

            for x in 1u64..10000 {
                let x = x as f64;

                assert_eq!(log(b, x), x.log(b));
            }
        }
    }
}
