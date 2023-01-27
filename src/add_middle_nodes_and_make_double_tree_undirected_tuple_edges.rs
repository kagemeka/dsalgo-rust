pub fn add_middle_nodes(edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let n = edges.len() + 1;

    let mut new_edges = Vec::with_capacity((n - 1) << 1);

    for i in 0..n - 1 {
        let (u, v) = edges[i];

        new_edges.push((u, n + i));

        new_edges.push((v, n + i));
    }

    new_edges
}
