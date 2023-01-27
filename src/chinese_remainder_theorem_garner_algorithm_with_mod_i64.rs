use crate::modular_inverse_euclidean_i64_no_error::modinv;

/// modulus of pairs must be pairwise coprime.
/// but taking modulus is not necessarily coprime with others or prime itself.
/// v_i = t_0 + t_1m_0 + t_2m_0m_1 + ... + t_i\prod_{j<i}{m_j}
/// t_i = (r_i - v_{i-1})(\prod_{j<i}{m_j})^{-1} ( mod  m_i) (v_{-1} = 0)

pub fn garner_with_mod(
    modulus: i64,
    mr: &[(i64, i64)],
) -> i64 {
    let n = mr.len();

    let mut mr = mr.to_vec();

    mr.push((modulus, 0)); // 0 is meaningless.
    let mut v = vec![0; n + 1]; // previsous value v_{i-1} (mod m_i)
    let mut m_prod = vec![1; n + 1]; // \prod_{j<i} m_j (mod m_i)
    for (i, &(m, r)) in mr.iter().enumerate().take(n) {
        let t = (r - v[i]) % m * modinv(m, m_prod[i]) % m;

        for ((v, p), x) in v[i + 1..]
            .iter_mut()
            .zip(m_prod[i + 1..].iter_mut())
            .zip(mr[i + 1..].iter())
        {
            *v += t * *p % x.0;

            *v %= x.0;

            *p *= m % x.0;

            *p %= x.0;
        }
    }

    if v[n] < 0 {
        v[n] + modulus
    } else {
        v[n]
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (vec![(5, 3), (7, 4), (8, 3)], (280, 123)),
            (vec![(10, 3), (13, 5)], (1_000_000_007, 83)),
            (vec![(15, 2), (17, 8)], (1_000_000_007, 212)),
            (vec![(15, 2), (17, 8)], (100, 12)),
            (vec![(15, 2), (17, 8)], (60, 32)),
            (
                vec![(221549, 80712), (699312, 320302), (496729, 140367)],
                (76959154983203952, 38774484298448350),
            ),
        ];

        for (mr, (modulus, ans)) in cases {
            assert_eq!(garner_with_mod(modulus, &mr), ans);
        }
    }
}
