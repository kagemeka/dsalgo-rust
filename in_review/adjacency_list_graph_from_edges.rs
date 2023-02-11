pub fn graph_from_edges(
    n: usize,
    edges: &[(usize, usize)],
    directed: bool,
) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];

    for &(u, v) in edges {
        g[u].push(v);

        if !directed {
            g[v].push(u);
        }
    }

    g
}
