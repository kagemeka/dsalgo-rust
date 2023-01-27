//! to avoid floating point error

pub fn polygon_area_x2(a: &[(i64, i64)]) -> i64 {
    let mut s = 0;

    let n = a.len();

    for i in 0..n {
        let j = (i + 1) % n;

        s += (a[i].0 - a[j].0) * (a[i].1 + a[j].1);
    }

    s.abs()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
