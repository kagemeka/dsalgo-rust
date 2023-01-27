pub fn dijkstra_dense(
    inf: i64,
    g: &[Vec<i64>],
    src: usize,
) -> Vec<i64> {
    let n = g.len();

    let mut dist = vec![inf; n];

    dist[src] = 0;

    let mut visited = vec![false; n];

    // loop {
    for _ in 0..n - 1 {
        let mut u = n;

        let mut du = inf;

        for i in 0..n {
            if visited[i] || dist[i] >= du {
                continue;
            }

            u = i;

            du = dist[i];
        }

        if u == n {
            break;
        }

        visited[u] = true;

        for v in 0..n {
            if g[u][v] == inf {
                continue;
            }

            dist[v] = dist[v].min(du + g[u][v]);
        }
    }

    dist
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
