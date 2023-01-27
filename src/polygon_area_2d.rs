pub fn polygon_area(a: &[(i64, i64)]) -> f64 {
    let mut s = 0;

    let n = a.len();

    for i in 0..n {
        let j = (i + 1) % n;

        s += (a[i].0 - a[j].0) * (a[i].1 + a[j].1);
    }

    s.abs() as f64 / 2.
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(vec![(0, 0), (3, 1), (2, 4)], 5.0)];

        for (a, ans) in cases {
            assert_eq!(polygon_area(&a), ans);
        }
    }
}
