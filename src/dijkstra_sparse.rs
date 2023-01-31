/// fill inf as std::u64::MAX.

pub fn dijkstra(
    g: &[Vec<(usize, u64)>],
    s: usize,
) -> Vec<u64> {
    use std::{
        cmp::Reverse,
        collections::BinaryHeap,
    };

    type Q = BinaryHeap<Reverse<(u64, usize)>>;

    const INF: u64 = std::u64::MAX;

    let n = g.len();

    let mut dist = vec![INF; n];

    dist[s] = 0;

    let mut q = Q::new();

    q.push(Reverse((0, s)));

    while let Some(Reverse((du, u))) = q.pop() {
        if du > dist[u] {
            continue;
        }

        for &(v, w) in &g[u] {
            let dv = du + w;

            if dv >= dist[v] {
                continue;
            }

            dist[v] = dv;

            q.push(Reverse((dv, v)));
        }
    }

    dist
}
