pub fn dijkstra(
    g: &[Vec<(usize, i64)>],
    mut dist: Vec<i64>,
) -> Vec<i64> {
    use std::cmp::Reverse;

    let n = g.len();

    assert_eq!(dist.len(), n);

    let mut que = std::collections::BinaryHeap::new();

    for (i, &d) in dist.iter().enumerate() {
        que.push(Reverse((d, i)));
    }

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
