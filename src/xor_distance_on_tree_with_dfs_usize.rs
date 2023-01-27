pub fn tree_dfs_xor_dist(g: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let n = g.len();

    let mut dist = vec![0; n];

    fn dfs(
        g: &[Vec<(usize, usize)>],
        dist: &mut [usize],
        u: usize,
        p: usize,
    ) {
        for &(v, w) in g[u].iter() {
            if v == p {
                continue;
            }

            dist[v] = dist[u] ^ w;

            dfs(g, dist, v, u);
        }
    }

    dfs(g, &mut dist, 0, n);

    dist
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
