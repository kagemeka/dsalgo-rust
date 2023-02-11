pub fn convex_hull(points: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut a = points.to_vec();

    a.sort_unstable();

    let is_ccw = |p0: &(i64, i64), p1: &(i64, i64), p2: &(i64, i64)| -> bool {
        let (x0, y0) = p0;

        let (x1, y1) = p1;

        let (x2, y2) = p2;

        (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0) >= 0
    };

    let mut h = Vec::new();

    // upper hull
    for p in a.iter() {
        let mut k = h.len();

        while k >= 2 && is_ccw(&h[k - 2], &h[k - 1], p) {
            h.pop();

            k -= 1;
        }

        h.push(*p);
    }

    // lower hull
    let m = h.len();

    for p in a.iter().rev().skip(1) {
        let mut k = h.len();

        while k > m && is_ccw(&h[k - 2], &h[k - 1], p) {
            h.pop();

            k -= 1;
        }

        h.push(*p);
    }

    h.pop();

    h
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
