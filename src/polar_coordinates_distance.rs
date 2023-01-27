pub fn polar_dist(
    r0: f64,
    theta_0: f64,
    r1: f64,
    theta_1: f64,
) -> f64 {
    (r0 * r0 + r1 * r1 - 2. * r0 * r1 * (theta_1 - theta_0).cos()).sqrt()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/abc168/tasks/abc168_c
        let pi = std::f64::consts::PI;

        let cases = vec![
            ((3., 4., 9., 0.), 5.0),
            ((3., 4., 10., 40.), 4.56425719433005567605),
        ];

        const EPS: f64 = 1e10;

        for ((r0, r1, h, m), ans) in cases {
            let t0 = (h * 60.0 + m) / 12.0 / 60.0 * 2.0 * pi;

            let t1 = m / 60.0 * 2.0 * pi;

            assert!((polar_dist(r0, t0, r1, t1) - ans).abs() < EPS);
        }
    }
}
