pub fn tree_diameter(g: &[Vec<usize>]) -> usize {
    let n = g.len();

    let mut max_depth = vec![n; n];

    fn dfs(
        g: &[Vec<usize>],
        d: &mut [usize],
        u: usize,
    ) -> usize {
        d[u] = 0;

        let mut dists = vec![0, 0];

        let mut res = 0;

        for &v in g[u].iter() {
            if d[v] != g.len() {
                continue;
            }

            res = res.max(dfs(g, d, v));

            dists.push(d[v] + 1);

            dists.sort();

            dists.reverse();

            dists.pop();
        }

        d[u] = dists[0];

        res.max(dists.iter().sum())
    }

    dfs(g, &mut max_depth, 0)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 10;

        let edges = vec![
            (6, 5),
            (2, 1),
            (1, 3),
            (3, 4),
            (7, 8),
            (0, 7),
            (0, 5),
            (0, 1),
            (8, 9),
        ];

        let mut g = vec![vec![]; n];

        for (u, v) in edges {
            g[u].push(v);

            g[v].push(u);
        }

        assert_eq!(tree_diameter(&g), 6);
    }
}
