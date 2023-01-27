pub fn edge_count(adj_bits: &[usize]) -> Vec<usize> {
    let g = adj_bits;

    let n = g.len();

    let n2 = 1 << n;

    let mut f = vec![0; n2];

    for s in 1..n2 {
        let i = s.trailing_zeros() as usize;

        f[s] = f[s & (s - 1)] + (s & g[i]).count_ones() as usize;
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // https://atcoder.jp/contests/abc213/tasks/abc213_g
    }
}
