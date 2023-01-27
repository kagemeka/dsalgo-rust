pub fn lcm_of_factorization(
    factors: &[Vec<(u64, usize)>]
) -> Vec<(u64, usize)> {
    use std::collections::BTreeMap;

    let mut cnt = BTreeMap::new();

    for f in factors.iter() {
        for &(p, e) in f {
            let c = cnt.entry(p).or_insert(0);

            *c = (*c).max(e);
        }
    }

    cnt.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![vec![(2, 2), (5, 1)], vec![(5, 1)], vec![(2, 1), (7, 1)]];

        assert_eq!(lcm_of_factorization(&a), [(2, 2), (5, 1), (7, 1)]);
    }
}
