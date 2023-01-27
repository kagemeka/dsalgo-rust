pub fn graph_from_weigted_edges<T: Clone>(
    n: usize,
    edges: &[(usize, usize, T)],
    directed: bool,
) -> Vec<Vec<(usize, T)>> {
    let mut g = vec![vec![]; n];

    for (u, v, w) in edges.iter() {
        g[*u].push((*v, w.clone()));

        if !directed {
            g[*v].push((*u, w.clone()));
        }
    }

    g
}
