pub fn dijkstra(
    inf: i64,
    g: &[Vec<(usize, i64)>],
    src: usize,
    modulus: usize,
) -> Vec<usize> {
    use std::collections::BinaryHeap;

    let n = g.len();

    let mut dist = vec![inf; n];

    dist[src] = 0;

    let mut que = BinaryHeap::new();

    que.push((0, src));

    let mut cnt = vec![0; n];

    cnt[src] = 1;

    while let Some((mut du, u)) = que.pop() {
        du *= -1;

        if du > dist[u] {
            continue;
        }

        for &(v, w) in g[u].iter() {
            let dv = du + w;

            if dv > dist[v] {
                continue;
            }

            if dv == dist[v] {
                cnt[v] += cnt[u];

                cnt[v] %= modulus;

                continue;
            }

            cnt[v] = cnt[u];

            dist[v] = dv;

            que.push((-dv, v));
        }
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {}
}
