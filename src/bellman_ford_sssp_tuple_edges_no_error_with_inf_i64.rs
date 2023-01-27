pub fn bellman_ford(
    inf: i64,
    v_size: usize,
    edges: &[(usize, usize, i64)],
    src: usize,
) -> Vec<i64> {
    let mut dist = vec![inf; v_size];

    dist[src] = 0;

    for t in 1..v_size << 1 {
        for &(u, v, w) in edges {
            if dist[u] == inf {
                continue;
            }

            let dv = if dist[u] == -inf { -inf } else { dist[u] + w };

            if dv >= dist[v] {
                continue;
            }

            dist[v] = if t >= v_size { -inf } else { dv };
        }
    }

    dist
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_negative_edge() {
        let inf = 1 << 10;

        let edges =
            vec![(0, 1, 2), (0, 2, 3), (1, 2, -5), (1, 3, 1), (2, 3, 2)];

        assert_eq!(bellman_ford(inf, 4, &edges, 1), vec![inf, 0, -5, -3]);
    }

    #[test]

    fn test_negative_cycle() {
        let inf = 1 << 10;

        let edges = vec![
            (0, 1, 2),
            (0, 2, 3),
            (1, 2, -5),
            (1, 3, 1),
            (2, 3, 2),
            (3, 1, 0),
        ];

        assert_eq!(bellman_ford(inf, 4, &edges, 0), [0, -inf, -inf, -inf]);
    }
}
