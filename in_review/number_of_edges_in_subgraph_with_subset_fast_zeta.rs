use crate::fast_zeta_transform_for_subset_additive_with_std::*;

pub fn edge_count(adj_bits: &[usize]) -> Vec<usize> {
    let g = adj_bits;

    let n = g.len();

    let n2 = 1 << n;

    let mut f = vec![0; n2];

    for i in 0..n {
        for j in 0..i {
            f[(1 << i) | (1 << j)] = g[i] >> j & 1;
        }
    }

    fast_zeta_subset(f)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // https://atcoder.jp/contests/abc213/tasks/abc213_g
    }
}
