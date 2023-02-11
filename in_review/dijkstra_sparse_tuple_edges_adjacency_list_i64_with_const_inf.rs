pub fn dijkstra<const INF: i64>(
    g: &[Vec<(usize, i64)>],
    src: usize,
) -> Vec<i64> {
    use std::cmp::Reverse;

    let n = g.len();

    let mut dist = vec![INF; n];

    dist[src] = 0;

    let mut que = std::collections::BinaryHeap::new();

    que.push(Reverse((0, src)));

    while let Some(Reverse((du, u))) = que.pop() {
        if du > dist[u] {
            continue;
        }

        for &(v, w) in g[u].iter() {
            let dv = du + w;

            if dv >= dist[v] {
                continue;
            }

            dist[v] = dv;

            que.push(Reverse((dv, v)));
        }
    }

    dist
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
