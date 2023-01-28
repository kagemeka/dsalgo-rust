use crate::bellman_ford_abstract_dp::bellman_ford_abstract;

pub fn bellman_ford(
    inf: i64,
    v_size: usize,
    edges: &[(usize, usize, i64)],
    src: usize,
) -> Vec<i64> {
    let mut dist = vec![inf; v_size];

    dist[src] = 0;

    let mut iter_cnt: usize = 0;

    let m = edges.len();

    bellman_ford_abstract(edges, |&(u, v, w)| -> bool {
        iter_cnt += 1;

        let t = (iter_cnt + m - 1) / m;

        if dist[u] == inf {
            return false;
        }

        let dv = if dist[u] == -inf { -inf } else { dist[u] + w };

        if dv >= dist[v] {
            return false;
        }

        dist[v] = if t >= v_size { -inf } else { dv };

        return true;
    });

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
