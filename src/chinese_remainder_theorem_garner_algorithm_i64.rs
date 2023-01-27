use crate::modular_inverse_euclidean_i64_no_error::modinv;

pub fn garner(mr: &[(i64, i64)]) -> i64 {
    let mut v = 0;

    let mut m_prod = 1;

    for &(m, r) in mr.iter() {
        let t = (r - v) % m * modinv(m, m_prod) % m;

        v += t * m_prod;

        m_prod *= m;
    }

    if v < 0 {
        v += m_prod;
    }

    debug_assert!(0 <= v && v < m_prod);

    v
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (vec![(5, 3), (7, 4), (8, 3)], 123),
            (vec![], 0),
            (
                vec![(221549, 80712), (699312, 320302), (496729, 140367)],
                38774484298448350,
            ),
        ];

        for (mr, ans) in cases {
            assert_eq!(garner(&mr), ans);
        }
    }
}
