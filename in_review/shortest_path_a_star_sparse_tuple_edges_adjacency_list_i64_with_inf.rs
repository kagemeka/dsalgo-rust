pub fn a_star<F: Fn(usize) -> i64>(
    inf: i64,
    g: &[Vec<(usize, i64)>],
    src: usize,
    dst: usize,
    heuristic_func: F,
) -> i64 {
    use std::cmp::Reverse;

    let n = g.len();

    let mut cost = vec![inf; n];

    cost[src] = 0;

    let mut que = std::collections::BinaryHeap::new();

    // (smaller score, larger cost (or smaller heuristic cost), node)
    que.push((Reverse(heuristic_func(src)), 0, src));

    while let Some((Reverse(_), cu, u)) = que.pop() {
        if u == dst {
            return cu;
        }

        if cu > cost[u] {
            continue;
        }

        for &(v, w) in g[u].iter() {
            let cv = cu + w;

            if cv >= cost[v] {
                continue;
            }

            cost[v] = cv;

            let sv = heuristic_func(v) + cv;

            que.push((Reverse(sv), cv, v));
        }
    }

    inf
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
