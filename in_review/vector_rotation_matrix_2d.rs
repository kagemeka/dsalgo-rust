pub fn rotation_matrix(radian: f64) -> [[f64; 2]; 2] {
    let s = radian.sin();

    let c = radian.cos();

    [[c, -s], [s, c]]
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use std::f64::consts::PI;

        let rad = PI / 2.;

        dbg!(rotation_matrix(rad));

        dbg!(rotation_matrix(-rad));
    }
}
