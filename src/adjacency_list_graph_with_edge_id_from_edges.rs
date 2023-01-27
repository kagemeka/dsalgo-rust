pub fn graph_from_edges(
    n: usize,
    edges: &[(usize, usize)],
    directed: bool,
) -> Vec<Vec<(usize, usize)>> {
    let mut g = vec![vec![]; n];

    for (i, &(u, v)) in edges.iter().enumerate() {
        g[u].push((v, i));

        if !directed {
            g[v].push((u, i));
        }
    }

    g
}
