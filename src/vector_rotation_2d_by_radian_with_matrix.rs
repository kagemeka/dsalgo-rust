//! don't confused with matrix rotation.
//! this is the matrix for rotation of vectors.

/// counter-clockwise.

pub fn rotate(
    x: f64,
    y: f64,
    radian_theta: f64,
) -> (f64, f64) {
    let c = radian_theta.cos();

    let s = radian_theta.sin();

    (x * c - y * s, x * s + y * c)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases: &[(f64, f64, f64)] = &[
            (2., 2., 180.),
            (5., 0., 120.),
            (0., 0., 11.),
            (15., 5., 360.),
            (-505., 191., 278.),
        ];

        let answers = &[
            (-2.0000000000000004, -1.9999999999999998),
            (-2.499999999999999, 4.330127018922194),
            (0.0, 0.0),
            (15.000000000000002, 4.9999999999999964),
            (118.85878514480687, 526.6674369978655),
        ];

        const TAU: f64 = std::f64::consts::TAU;

        let eps = 1e-6;

        for (&(x, y, deg), &ans) in cases.iter().zip(answers) {
            let theta = deg / 360. * TAU;

            // println!("{:#?}", rotate(x, y, theta));
            let (x, y) = rotate(x, y, theta);

            let (ax, ay) = ans;

            let (dx, dy) = ((x - ax).abs(), (y - ay).abs());

            assert!(dx < eps || (dx / ax).abs() < eps);

            assert!(dy < eps || (dy / ay).abs() < eps);
        }
    }
}
